//! Adaptive thresholding


use self::super::super::super::ops::{QUIRC_PERSPECTIVE_PARAMS, QUIRC_MAX_CAPSTONES, QUIRC_MAX_REGIONS, QUIRC_MAX_GRIDS, QuircCapstone, QuircRegion, QuircPoint,
                                     QuircCode, QuircGrid, Quirc};
use self::super::super::super::ops::version_db::{QUIRC_MAX_ALIGNMENT, QUIRC_MAX_VERSION, QUIRC_VERSION_DB};
use self::super::{perspective_setup, perspective_unmap, perspective_map, line_intersect};
use self::super::super::super::constants::QUIRC_MAX_BITMAP;
use std::{mem, ptr};


const THRESHOLD_S_MIN: usize = 1;
const THRESHOLD_S_DEN: usize = 8;
const THRESHOLD_T: usize = 5;

const QUIRC_PIXEL_WHITE: u16 = 0;
const QUIRC_PIXEL_BLACK: u16 = 1;
const QUIRC_PIXEL_REGION: u16 = 2;


impl Quirc {
    fn threshold(&mut self) {
        let mut avg_w = 0usize;
        let mut avg_u = 0usize;
        let mut threshold_s = self.w / THRESHOLD_S_DEN;

        /* Ensure a sane, non-zero value for threshold_s.
         *
         * threshold_s can be zero if the image width is small. We need to avoid
         * SIGFPE as it will be used as divisor.
         * */
        threshold_s = threshold_s.max(THRESHOLD_S_MIN);

        for y in 0..self.h {
            let mut row = &mut self.pixels[y + 1 * self.w..];

            // There has to be a better way of doing this
            unsafe {
                ptr::write_bytes(self.row_average.as_mut_ptr(), 0u8, self.row_average.len());
            }

            for x in 0..self.w {
                let (w, u) = if y & 1 == 1 {
                    (x, self.w - 1 - x)
                } else {
                    (self.w - 1 - x, x)
                };

                avg_w = (avg_w * (threshold_s - 1)) / threshold_s + row[w] as usize;
                avg_u = (avg_u * (threshold_s - 1)) / threshold_s + row[u] as usize;

                self.row_average[w] += avg_w as u64;
                self.row_average[u] += avg_u as u64;
            }

            for x in 0..self.w {
                if (row[x] as u64) < self.row_average[x] * ((100 - THRESHOLD_T) as u64) / ((200 * threshold_s) as u64) {
                    row[x] = QUIRC_PIXEL_BLACK;
                } else {
                    row[x] = QUIRC_PIXEL_WHITE;
                }
            }
        }
    }


    fn region_code(&mut self, x: usize, y: usize) -> Option<usize> {
        if x >= self.w || y >= self.h {
            return None;
        }

        let pixel = self.pixels[y * self.w + x];

        if pixel >= QUIRC_PIXEL_REGION {
            return Some(pixel as usize);
        }

        if pixel == QUIRC_PIXEL_WHITE {
            return None;
        }

        if self.num_regions >= QUIRC_MAX_REGIONS {
            return None;
        }

        let region = self.num_regions;

        let mut count = 0u32;
        self.flood_fill_seed(x, y, pixel, region as u16, |_, left, right| count += (right - left + 1) as u32, 0);

        self.num_regions += 1;
        self.regions[self.num_regions] = QuircRegion {
            seed: QuircPoint {
                x: x as isize,
                y: y as isize,
            },
            count: count,
            capstone: -1,
        };

        return Some(region);
    }

    fn find_region_corners(&mut self, rcode: usize, mut reference: QuircPoint, corners: &mut [QuircPoint]) {
        assert!(corners.len() >= 4);

        let region = self.regions[rcode];
        let mut scores = [0i64; 4];

        // find_one_corner
        self.flood_fill_seed(region.seed.x as usize,
                             region.seed.y as usize,
                             rcode as u16,
                             QUIRC_PIXEL_BLACK,
                             |y, left, right| {
            let xs = [left, right];
            let dy = y as isize - reference.y;

            for i in 0..2 {
                let dx = xs[i] as isize - reference.x;
                let d = dx * dx + dy * dy;

                if d > scores[0] as isize {
                    scores[0] = d as i64;
                    corners[0].x = xs[i] as isize;
                    corners[0].y = y as isize;
                }
            }
        },
                             0);

        reference.x = corners[0].x - reference.x;
        reference.y = corners[0].y - reference.y;

        for mut i in 0..4 {
            corners[i] = region.seed;

            i = (region.seed.x * reference.x + region.seed.y * reference.y) as usize;
            scores[0] = i as i64;
            scores[2] = -(i as i64);
            i = (region.seed.x * -reference.y + region.seed.y * reference.x) as usize;
            scores[1] = i as i64;
            scores[3] = -(i as i64);

            // find_other_corners
            self.flood_fill_seed(region.seed.x as usize,
                                 region.seed.y as usize,
                                 QUIRC_PIXEL_BLACK,
                                 rcode as u16,
                                 |y, left, right| {
                let xs = [left, right];

                for i in 0..2 {
                    let up = xs[i] as isize * reference.x + y as isize * reference.y;
                    let right = xs[i] as isize * -reference.y + y as isize * reference.x;
                    let in_scores = [up, right, -up, -right];

                    for j in 0..4 {
                        if in_scores[j] > scores[j] as isize {
                            scores[j] = in_scores[j] as i64;
                            corners[j].x = xs[i] as isize;
                            corners[j].y = y as isize;
                        }
                    }
                }
            },
                                 0);
        }
    }

    fn record_capstone(&mut self, ring: usize, stone: usize) {
        if self.num_capstones >= QUIRC_MAX_CAPSTONES {
            return;
        }

        let mut capstone = QuircCapstone {
            ring: ring as u32,
            stone: stone as u32,

            corners: [QuircPoint { x: 0, y: 0 }; 4],
            center: QuircPoint { x: 0, y: 0 },
            c: [0f64; QUIRC_PERSPECTIVE_PARAMS],

            qr_grid: -1,
        };

        self.regions[stone].capstone = self.num_capstones as i32;
        self.regions[ring].capstone = self.num_capstones as i32;

        // Find the corners of the ring
        let stone_seed = self.regions[stone].seed;
        self.find_region_corners(ring, stone_seed, &mut capstone.corners);

        // Set up the perspective transform and find the center
        capstone.c = perspective_setup(&capstone.corners, 7.0, 7.0);
        capstone.center = perspective_map(&capstone.c, 3.5, 3.5);

        self.capstones[self.num_capstones] = capstone;
        self.num_capstones += 1;
    }

    fn test_capstone(&mut self, x: usize, y: usize, pb: &[usize]) {
        macro_rules! try_opt {
            ($opt:expr) => {
                if let Some(data) = $opt {
                    data
                } else {
                    return
                }
            }
        }

        let ring_right = try_opt!(self.region_code(x - pb[4], y));
        let stone = try_opt!(self.region_code(x - pb[4] - pb[3] - pb[2], y));
        let ring_left = try_opt!(self.region_code(x - pb[4] - pb[3] - pb[2] - pb[1] - pb[0], y));

        // Left and ring of ring should be connected
        if ring_left != ring_right {
            return;
        }

        // Ring should be disconnected from stone
        if ring_left == stone {
            return;
        }

        {
            let stone_reg = &self.regions[stone];
            let ring_reg = &self.regions[ring_left];

            /* Already detected */
            if stone_reg.capstone >= 0 || ring_reg.capstone >= 0 {
                return;
            }

            /* Ratio should ideally be 37.5 */
            let ratio = stone_reg.count * 100 / ring_reg.count;
            if ratio < 10 || ratio > 70 {
                return;
            }
        }

        self.record_capstone(ring_left, stone);
    }

    fn finder_scan(&mut self, y: usize) {
        static CHECK: [usize; 5] = [1, 1, 3, 1, 1];

        let mut last_color = false;
        let mut run_length = 0;
        let mut run_count = 0;
        let mut pb = [0usize; 5];

        for x in 0..self.w {
            let color = self.pixels[y * self.w + x] != 0;

            if x != 0 && color != last_color {
                unsafe {
                    ptr::copy(pb[1..].as_ptr(), pb.as_mut_ptr(), 4);
                }
                pb[4] = run_length;

                run_length = 0;
                run_count += 1;

                if !color && run_count >= 5 {
                    let mut ok = true;

                    let avg = (pb[0] + pb[1] + pb[3] + pb[4]) / 4;
                    let err = avg * 3 / 4;

                    for i in 0..5 {
                        if (pb[i] < CHECK[i] * avg - err) || (pb[i] > CHECK[i] * avg + err) {
                            ok = false;
                        }
                    }

                    if ok {
                        self.test_capstone(x, y, &pb);
                    }
                }
            }

            run_length += 1;
            last_color = color;
        }
    }

    fn find_alignment_pattern(&mut self, index: usize) {
        let c0 = self.capstones[self.grids[index].caps[0] as usize].c;
        let c2 = self.capstones[self.grids[index].caps[2] as usize].c;
        let mut step_size = 1;
        let mut dir = 0;

        // Grab our previous estimate of the alignment pattern corner
        let mut b = self.grids[index].align;

        // Guess another two corners of the alignment pattern so that we
        // can estimate its size.
        let (u, v) = perspective_unmap(&c0, &b);
        let a = perspective_map(&c0, u, v + 1.0);
        let (u, v) = perspective_unmap(&c2, &b);
        let c = perspective_map(&c2, u + 1.0, v);

        let size_estimate = ((a.x - b.x) * -(c.y - b.y) + (a.y - b.y) * (c.x - b.x)).abs();

        // Spiral outwards from the estimate point until we find something
        // roughly the right size. Don't look too far from the estimate
        // point.
        while step_size * step_size < size_estimate * 100 {
            static DX_MAP: [isize; 4] = [1, 0, -1, 0];
            static DY_MAP: [isize; 4] = [0, -1, 0, 1];

            for _ in 0..step_size {
                if let Some(code) = self.region_code(b.x as usize, b.y as usize) {
                    let reg = &self.regions[code];

                    if (reg.count as isize >= size_estimate / 2) && (reg.count as isize <= size_estimate * 2) {
                        self.grids[index].align_region = code as i32;
                        return;
                    }
                }

                b.x += DX_MAP[dir];
                b.y += DY_MAP[dir];
            }

            dir = (dir + 1) % 4;
            if dir & 1 == 0 {
                step_size += 1;
            }
        }
    }

    /// Do a Bresenham scan from one point to another and count the number
    /// of black/white transitions.
    fn timing_scan(&self, p0: &QuircPoint, p1: &QuircPoint) -> Option<usize> {
        let mut n = p1.x - p0.x;
        let mut d = p1.y - p0.y;
        let mut x = p0.x;
        let mut y = p0.y;
        let mut a = 0;
        let mut run_length = 0;
        let mut count = 0;

        if p0.x < 0 || p0.y < 0 || p0.x >= self.w as isize || p0.y >= self.h as isize {
            return None;
        }
        if p1.x < 0 || p1.y < 0 || p1.x >= self.w as isize || p1.y >= self.h as isize {
            return None;
        }

        let (dom, nondom) = if n.abs() > d.abs() {
            mem::swap(&mut n, &mut d);
            (&mut x as *mut isize, &mut y as *mut isize)
        } else {
            (&mut y as *mut isize, &mut x as *mut isize)
        };
        let (dom, nondom) = unsafe { (&mut *dom, &mut *nondom) }; // save me from this nothing I've become

        let nondom_step = if n < 0 {
            n = -n;
            -1
        } else {
            1
        };

        let dom_step = if d < 0 {
            d = -d;
            -1
        } else {
            1
        };

        x = p0.x;
        y = p0.y;
        for _ in 0..(d + 1) {
            if y < 0 || y >= self.h as isize || x < 0 || x >= self.w as isize {
                break;
            }

            if self.pixels[(y * self.w as isize + x) as usize] != 0 {
                if run_length >= 2 {
                    count += 1;
                }
                run_length = 0;
            } else {
                run_length += 1;
            }

            a += n;
            *dom += dom_step;
            if a >= d {
                *nondom += nondom_step;
                a -= d;
            }
        }

        return Some(count);
    }

    /// Try the measure the timing pattern for a given QR code. This does
    /// not require the global perspective to have been set up, but it
    /// does require that the capstone corners have been set to their
    /// canonical rotation.
    ///
    /// For each capstone, we find a point in the middle of the ring band
    /// which is nearest the centre of the code. Using these points, we do
    /// a horizontal and a vertical timing scan.
    fn measure_timing_pattern(&mut self, index: usize) -> i32 {
        let mut qr = self.grids[index];

        for i in 0..3 {
            static US: [f64; 3] = [6.5, 6.5, 0.5];
            static VS: [f64; 3] = [0.5, 6.5, 6.5];

            qr.tpep[i] = perspective_map(&self.capstones[qr.caps[i] as usize].c, US[i], VS[i]);
        }

        qr.hscan = self.timing_scan(&qr.tpep[1], &qr.tpep[2]).map(|v| v as i32).unwrap_or(-1);
        qr.vscan = self.timing_scan(&qr.tpep[1], &qr.tpep[0]).map(|v| v as i32).unwrap_or(-1);

        let mut scan = qr.hscan;
        if qr.vscan > scan {
            scan = qr.vscan;
        }

        /* If neither scan worked, we can't go any further. */
        if scan < 0 {
            self.grids[index] = qr;
            return -1;
        }

        /* Choose the nearest allowable grid size */
        let size = scan * 2 + 13;
        let ver = (size - 15) / 4;
        qr.grid_size = (ver * 4 + 17) as u32;

        self.grids[index] = qr;
        return 0;
    }

    /// Read a cell from a grid using the currently set perspective
    /// transform. Returns +/- 1 for black/white, 0 for cells which are
    /// out of image bounds.
    fn read_cell(&self, index: usize, x: usize, y: usize) -> i8 {
        let p = perspective_map(&self.grids[index].c, x as f64 + 0.5, y as f64 + 0.5);
        if p.y < 0 || p.y >= self.h as isize || p.x < 0 || p.x >= self.w as isize {
            0
        } else if self.pixels[(p.y * self.w as isize + p.x) as usize] != 0 {
            1
        } else {
            -1
        }
    }

    fn fitness_cell(&self, index: usize, x: usize, y: usize) -> i64 {
        let mut score = 0i64;

        for v in 0..3 {
            for u in 0..3 {
                static OFFSETS: [f64; 3] = [0.3, 0.5, 0.7];

                let p = perspective_map(&self.grids[index].c, x as f64 + OFFSETS[u], y as f64 + OFFSETS[v]);
                if p.y < 0 || p.y >= self.h as isize || p.x < 0 || p.x >= self.w as isize {
                    continue;
                }

                if self.pixels[(p.y * self.w as isize + p.x) as usize] != 0 {
                    score += 1;
                } else {
                    score -= 1;
                }
            }
        }

        return score;
    }

    fn fitness_ring(&self, index: usize, cx: usize, cy: usize, radius: usize) -> i64 {
        let mut score = 0;
        for i in 0..radius * 2 {
            score += self.fitness_cell(index, cx - radius + i, cy - radius);
            score += self.fitness_cell(index, cx - radius, cy + radius - i);
            score += self.fitness_cell(index, cx + radius, cy - radius + i);
            score += self.fitness_cell(index, cx + radius - i, cy + radius);
        }
        score
    }

    fn fitness_apat(&self, index: usize, cx: usize, cy: usize) -> i64 {
        self.fitness_cell(index, cx, cy) - self.fitness_ring(index, cx, cy, 1) + self.fitness_ring(index, cx, cy, 2)
    }

    fn fitness_capstone(&self, index: usize, mut x: usize, mut y: usize) -> i64 {
        x += 3;
        y += 3;

        self.fitness_cell(index, x, y) + self.fitness_ring(index, x, y, 1) - self.fitness_ring(index, x, y, 2) + self.fitness_ring(index, x, y, 3)
    }

    /// Compute a fitness score for the currently configured perspective
    /// transform, using the features we expect to find by scanning the
    /// grid.
    fn fitness_all(&self, index: usize) -> i64 {
        let qr = &self.grids[index];
        let version = (qr.grid_size - 17) / 4;
        let info = &QUIRC_VERSION_DB[version as usize];
        let mut score = 0i64;

        // Check the timing pattern
        for i in 0..qr.grid_size {
            let expect = if i & 1 == 1 { 1 } else { -1 };

            score += self.fitness_cell(index, (i as usize) + 7, 6) * expect;
            score += self.fitness_cell(index, 6, (i as usize) + 7) * expect;
        }

        // Check capstones
        score += self.fitness_capstone(index, 0, 0);
        score += self.fitness_capstone(index, (qr.grid_size - 7) as usize, 0);
        score += self.fitness_capstone(index, 0, (qr.grid_size - 7) as usize);

        if version as usize > QUIRC_MAX_VERSION {
            return score;
        }

        // Check alignment patterns
        let mut ap_count = 0;
        while (ap_count < QUIRC_MAX_ALIGNMENT) && info.apat[ap_count] != 0 {
            ap_count += 1;
        }

        for i in 1..ap_count - 1 {
            score += self.fitness_apat(index, 6, info.apat[i] as usize);
            score += self.fitness_apat(index, info.apat[i] as usize, 6);
        }

        for i in 1..ap_count {
            for j in 1..ap_count {
                score += self.fitness_apat(index, info.apat[i] as usize, info.apat[j] as usize);
            }
        }

        return score;
    }

    fn jiggle_perspective(&mut self, index: usize) {
        let mut best = self.fitness_all(index);
        let mut adjustments = [0f64; 8];

        {
            let qr = &self.grids[index];
            for i in 0..8 {
                adjustments[i] = qr.c[i] * 0.02;
            }
        }

        for _ in 0..5 {
            for i in 0..16 {
                let j = i >> 1;
                let old = self.grids[index].c[j];
                let step = adjustments[j];

                let new = {
                    if i & 1 == 1 { old + step } else { old - step }
                };

                self.grids[index].c[j] = new;
                let test = self.fitness_all(index);

                if test > best {
                    best = test;
                } else {
                    self.grids[index].c[j] = old;
                }
            }

            for i in 0..8 {
                adjustments[i] *= 0.5;
            }
        }
    }

    /// Once the capstones are in place and an alignment point has been
    /// chosen, we call this function to set up a grid-reading perspective
    /// transform.
    fn setup_qr_perspective(&mut self, index: usize) {
        {
            let qr = &mut self.grids[index];
            // Set up the perspective map for reading the grid
            qr.c = perspective_setup(&[self.capstones[qr.caps[1] as usize].corners[0], // align
                                       self.capstones[qr.caps[2] as usize].corners[0], // align
                                       qr.align, // align
                                       self.capstones[qr.caps[0] as usize].corners[0]],
                                     (qr.grid_size - 7) as f64,
                                     (qr.grid_size - 7) as f64);
        }

        self.jiggle_perspective(index);
    }

    fn record_qr_grid(&mut self, mut a: u32, b: u32, mut c: u32) {
        if self.num_grids >= QUIRC_MAX_GRIDS {
            return;
        }

        // Construct the hypotenuse line from A to C. B should be to
        // the left of this line.
        let h0 = self.capstones[a as usize].center;
        let mut hd = QuircPoint {
            x: self.capstones[c as usize].center.x - self.capstones[a as usize].center.x,
            y: self.capstones[c as usize].center.y - self.capstones[a as usize].center.y,
        };

        // Make sure A-B-C is clockwise
        if (self.capstones[b as usize].center.x - h0.x) * -hd.y + (self.capstones[b as usize].center.y - h0.y) * hd.x > 0 {
            mem::swap(&mut a, &mut c);

            hd.x = -hd.x;
            hd.y = -hd.y;
        }

        // Record the grid and its components
        let qr_index = self.num_grids;
        self.num_grids += 1;

        let mut qr = QuircGrid {
            caps: [a, b, c],

            align_region: -1,
            align: QuircPoint { x: 0, y: 0 },

            tpep: [QuircPoint { x: 0, y: 0 }; 3],
            hscan: 0,
            vscan: 0,

            grid_size: 0,
            c: [0f64; QUIRC_PERSPECTIVE_PARAMS],
        };
        self.grids[self.num_grids] = qr;

        // Rotate each capstone so that corner 0 is top-left with respect
        // to the grid.
        for i in 0..3 {
            let cap = &mut self.capstones[qr.caps[i] as usize];
            cap.rotate_capstone(&h0, &hd);
            cap.qr_grid = qr_index as i32;
        }

        // Check the timing pattern. This doesn't require a perspective
        // transform.
        if self.measure_timing_pattern(qr_index) < 0 ||
           // Make an estimate based for the alignment pattern based on extending
           // lines from capstones A and C.
           line_intersect(&self.capstones[a as usize].corners[0],
             &self.capstones[a as usize].corners[1],
             &self.capstones[c as usize].corners[0],
             &self.capstones[c as usize].corners[3]).is_none() {
            // We've been unable to complete setup for this grid. Undo what we've
            // recorded and pretend it never happened.
            for i in 0..3 {
                self.capstones[qr.caps[i] as usize].qr_grid = -1;
            }
            self.num_grids -= 1;
            return;
        }

        // On V2+ grids, we should use the alignment pattern.
        if qr.grid_size > 21 {
            // Try to find the actual location of the alignment pattern.
            self.find_alignment_pattern(qr_index);

            // Find the point of the alignment pattern closest to the
            // top-left of the QR grid.
            if qr.align_region >= 0 {
                let reg_seed = self.regions[qr.align_region as usize].seed;

                // Start from some point inside the alignment pattern
                self.grids[self.num_grids].align = reg_seed;
                qr.align = reg_seed;

                let mut score = -hd.y * qr.align.x + hd.x * qr.align.y;

                self.flood_fill_seed(reg_seed.x as usize,
                                     reg_seed.y as usize,
                                     qr.align_region as u16,
                                     QUIRC_PIXEL_BLACK,
                                     |_, _, _| {},
                                     0);
                self.flood_fill_seed(reg_seed.x as usize,
                                     reg_seed.y as usize,
                                     QUIRC_PIXEL_BLACK,
                                     qr.align_region as u16,
                                     |y, left, right| {
                    let xs = [left, right];

                    for i in 0..2 {
                        let d = -hd.y * (xs[i] as isize) + hd.x * (y as isize);

                        if d < score {
                            score = d;
                            qr.align.x = xs[i] as isize;
                            qr.align.y = y as isize;
                        }
                    }
                },
                                     0);
            }
        }

        self.setup_qr_perspective(qr_index);
    }

    fn test_neighbours(&mut self, i: u32, hlist: &NeighbourList, vlist: &NeighbourList) {
        let mut best_score = 0f64;
        let mut best_h = -1f64;
        let mut best_v = -1f64;

        // Test each possible grouping
        for j in 0..hlist.count {
            for k in 0..vlist.count {
                let hn = &hlist.n[j];
                let vn = &vlist.n[k];
                let score = (1.0 - hn.distance / vn.distance).abs();

                if score > 2.5 {
                    continue;
                }

                if best_h < 0f64 || score < best_score {
                    best_h = hn.index as f64;
                    best_v = vn.index as f64;
                    best_score = score;
                }
            }
        }

        if best_h < 0 as f64 || best_v < 0 as f64 {
            return;
        }

        self.record_qr_grid(best_h as u32, i, best_v as u32);
    }

    fn test_grouping(&mut self, i: u32) {
        if self.capstones[i as usize].qr_grid >= 0 {
            return;
        }

        let mut hlist = NeighbourList {
            n: [Neighbour {
                index: 0,
                distance: 0f64,
            }; QUIRC_MAX_CAPSTONES],
            count: 0,
        };
        let mut vlist = hlist;

        // Look for potential neighbours by examining the relative gradients
        // from this capstone to others.
        for j in 0..self.num_capstones {
            let c2 = &self.capstones[j];

            if i as usize == j || c2.qr_grid >= 0 {
                continue;
            }

            let (mut u, mut v) = perspective_unmap(&self.capstones[i as usize].c, &c2.center);

            u = (u - 3.5).abs();
            v = (v - 3.5).abs();

            if u < 0.2 * v {
                let n = &mut hlist.n[hlist.count];
                hlist.count += 1;

                n.index = j;
                n.distance = v;
            }

            if v < 0.2 * u {
                let n = &mut vlist.n[vlist.count];
                vlist.count += 1;

                n.index = j;
                n.distance = u;
            }
        }

        if !(hlist.count != 0 && vlist.count != 0) {
            return;
        }

        self.test_neighbours(i, &hlist, &vlist);
    }

    fn pixels_setup(&mut self) {
        for y in 0..self.h {
            for x in 0..self.w {
                self.pixels[y * self.w + x] = self.image[y * self.w + x] as u16;
            }
        }
    }

    /// These functions are used to process images for QR-code recognition –
    /// `begin()` be called first to obtain access to a buffer into
    /// which the input image should be placed.
    pub fn begin(&mut self) -> &mut [u8] {
        self.num_regions = QUIRC_PIXEL_REGION as usize;
        self.num_capstones = 0;
        self.num_grids = 0;

        return &mut self.image;
    }

    /// After filling the buffer, `end()` should be called to process
    /// the image for QR-code recognition.
    pub fn end(&mut self) {
        self.pixels_setup();
        self.threshold();

        for i in 0..self.h {
            self.finder_scan(i);
        }

        for i in 0..self.num_capstones {
            self.test_grouping(i as u32);
        }
    }

    /// Extract the QR-code specified by the given index
    pub fn extract(&self, index: usize) -> Option<QuircCode> {
        if index > self.num_grids {
            return None;
        }

        let qr = &self.grids[index];

        let mut code = QuircCode {
            corners: [perspective_map(&qr.c, 0.0, 0.0),
                      perspective_map(&qr.c, qr.grid_size as f64, 0.0),
                      perspective_map(&qr.c, qr.grid_size as f64, qr.grid_size as f64),
                      perspective_map(&qr.c, 0.0, qr.grid_size as f64)],
            size: qr.grid_size,
            cell_bitmap: [0u8; QUIRC_MAX_BITMAP],
        };


        let mut i = 0u32;
        for y in 0..qr.grid_size {
            for x in 0..qr.grid_size {
                if self.read_cell(index, x as usize, y as usize) > 0 {
                    code.cell_bitmap[(i >> 3) as usize] |= 1 << (i & 7);
                }

                i += 1;
            }
        }

        Some(code)
    }
}

impl QuircCapstone {
    /// Rotate the capstone with so that corner 0 is the leftmost with respect
    /// to the given reference line.
    fn rotate_capstone(&mut self, h0: &QuircPoint, hd: &QuircPoint) {
        let mut copy = [QuircPoint { x: 0, y: 0 }; 4];
        let mut best = 0;
        let mut best_score = 0i64;

        for j in 0..4 {
            let p = &self.corners[j];
            let score = (p.x - h0.x) * -hd.y + (p.y - h0.y) * hd.x;

            if j == 0 || score < best_score as isize {
                best = j;
                best_score = score as i64;
            }
        }

        // Rotate the capstone
        for j in 0..4 {
            copy[j] = self.corners[((j + best) % 4) as usize];
        }
        self.corners = copy;
        self.c = perspective_setup(&self.corners, 7.0, 7.0);
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
struct Neighbour {
    index: usize,
    distance: f64,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
struct NeighbourList {
    n: [Neighbour; QUIRC_MAX_CAPSTONES],
    count: usize,
}
