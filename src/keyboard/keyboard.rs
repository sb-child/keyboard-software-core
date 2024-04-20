use super::key::Key;

#[derive(Debug, Clone, Copy)]
pub struct Keyboard {
  keys: [Key; 110],
}

impl Keyboard {
  pub fn new() -> Self {
    Self {
      keys: [
        // chunk 1
        Key::new(0, Some(0), 14.526, 14.526),
        Key::new(1, Some(0), 38.318, 14.526),
        Key::new(2, Some(0), 14.526, 43.101),
        Key::new(3, Some(0), 38.318, 43.101),
        Key::new(4, Some(0), 57.388, 43.101),
        Key::new(5, Some(0), 14.526, 62.151),
        Key::new(6, Some(0), 43.101, 62.151),
        Key::new(7, Some(0), 66.913, 62.151),
        // chunk 2
        Key::new(8, Some(0), 76.438, 14.526),
        Key::new(9, Some(0), 96.028, 14.526),
        Key::new(10, Some(0), 114.538, 14.526),
        Key::new(11, Some(0), 133.588, 14.526),
        Key::new(12, Some(0), 76.438, 43.101),
        Key::new(13, Some(0), 95.488, 43.101),
        Key::new(14, Some(0), 114.538, 43.101),
        Key::new(15, Some(0), 133.588, 43.101),
        Key::new(16, Some(0), 85.963, 62.151),
        Key::new(17, Some(0), 105.013, 62.151),
        Key::new(18, Some(0), 124.063, 62.151),
        Key::new(19, Some(0), 143.113, 62.151),
        // chunk 3
        Key::new(20, Some(0), 162.163, 14.526),
        Key::new(21, Some(0), 181.213, 14.526),
        Key::new(22, Some(0), 200.263, 14.526),
        Key::new(23, Some(0), 219.313, 14.526),
        Key::new(24, Some(0), 152.638, 43.101),
        Key::new(25, Some(0), 171.688, 43.101),
        Key::new(26, Some(0), 190.738, 43.101),
        Key::new(27, Some(0), 209.788, 43.101),
        Key::new(28, Some(0), 228.838, 43.101),
        Key::new(29, Some(0), 162.163, 62.151),
        Key::new(30, Some(0), 181.213, 62.151),
        Key::new(31, Some(0), 200.263, 62.151),
        Key::new(32, Some(0), 219.313, 62.151),
        // chunk 4
        Key::new(33, Some(0), 247.888, 14.526),
        Key::new(34, Some(0), 266.938, 14.526),
        Key::new(35, Some(0), 285.988, 14.526),
        Key::new(36, Some(0), 305.038, 14.526),
        Key::new(37, Some(0), 247.888, 43.101),
        Key::new(38, Some(0), 266.938, 43.101),
        Key::new(39, Some(0), 295.513, 43.101),
        Key::new(40, Some(0), 238.363, 62.151),
        Key::new(41, Some(0), 257.413, 62.151),
        Key::new(42, Some(0), 276.463, 62.151),
        Key::new(43, Some(0), 300.276, 62.151),
        // chunk 5
        Key::new(44, Some(0), 328.851, 14.526),
        Key::new(45, Some(0), 347.901, 14.526),
        Key::new(46, Some(0), 366.951, 14.526),
        Key::new(47, Some(0), 328.851, 43.101),
        Key::new(48, Some(0), 347.901, 43.101),
        Key::new(49, Some(0), 366.951, 43.101),
        Key::new(50, Some(0), 328.851, 62.151),
        Key::new(51, Some(0), 347.901, 62.151),
        Key::new(52, Some(0), 366.951, 62.151),
        // chunk 6
        Key::new(53, Some(0), 390.763, 43.101),
        Key::new(54, Some(0), 409.813, 43.101),
        Key::new(55, Some(0), 428.863, 43.101),
        Key::new(56, Some(0), 447.913, 43.101),
        Key::new(57, Some(0), 390.763, 62.151),
        Key::new(58, Some(0), 409.813, 62.151),
        Key::new(59, Some(0), 428.863, 62.151),
        // chunk 7
        Key::new(60, Some(0), 14.526, 81.201),
        Key::new(61, Some(0), 45.482, 81.201),
        Key::new(62, Some(0), 14.526, 100.251),
        Key::new(63, Some(0), 50.244, 100.251),
        Key::new(64, Some(0), 14.526, 119.301),
        Key::new(65, Some(0), 40.719, 119.301),
        Key::new(66, Some(0), 64.532, 119.301),
        // chunk 8
        Key::new(67, Some(0), 71.676, 81.201),
        Key::new(68, Some(0), 90.726, 81.201),
        Key::new(69, Some(0), 109.776, 81.201),
        Key::new(70, Some(0), 128.826, 81.201),
        Key::new(71, Some(0), 81.201, 100.251),
        Key::new(72, Some(0), 100.251, 100.251),
        Key::new(73, Some(0), 119.301, 100.251),
        Key::new(74, Some(0), 138.351, 100.251),
        Key::new(75, Some(0), 88.344, 119.301),
        // chunk 9
        Key::new(76, Some(0), 147.876, 81.201),
        Key::new(77, Some(0), 166.926, 81.201),
        Key::new(78, Some(0), 185.976, 81.201),
        Key::new(79, Some(0), 205.026, 81.201),
        Key::new(80, Some(0), 224.076, 81.201),
        Key::new(81, Some(0), 157.401, 100.251),
        Key::new(82, Some(0), 176.451, 100.251),
        Key::new(83, Some(0), 195.501, 100.251),
        Key::new(84, Some(0), 214.551, 100.251),
        Key::new(85, Some(0), 159.782, 119.301),
        // chunk 10
        Key::new(86, Some(0), 243.126, 81.201),
        Key::new(87, Some(0), 262.176, 81.201),
        Key::new(88, Some(0), 293.132, 81.201),
        Key::new(89, Some(0), 233.601, 100.251),
        Key::new(90, Some(0), 252.650, 100.251),
        Key::new(91, Some(0), 288.369, 100.251),
        Key::new(92, Some(0), 231.219, 119.301),
        Key::new(93, Some(0), 255.032, 119.301),
        Key::new(94, Some(0), 278.844, 119.301),
        Key::new(95, Some(0), 302.657, 119.301),
        // chunk 11
        Key::new(96, Some(0), 347.901, 100.251),
        Key::new(97, Some(0), 328.851, 119.301),
        Key::new(98, Some(0), 347.901, 119.301),
        Key::new(99, Some(0), 366.951, 119.301),
        // chunk 12
        Key::new(100, Some(0), 447.913, 71.676),
        Key::new(101, Some(0), 390.763, 81.201),
        Key::new(102, Some(0), 409.813, 81.201),
        Key::new(103, Some(0), 428.863, 81.201),
        Key::new(104, Some(0), 390.763, 100.251),
        Key::new(105, Some(0), 409.813, 100.251),
        Key::new(106, Some(0), 428.863, 100.251),
        Key::new(107, Some(0), 447.913, 109.776),
        Key::new(108, Some(0), 400.288, 119.301),
        Key::new(109, Some(0), 428.863, 119.301),
      ],
    }
  }
  // def distance_to_circle(x, y, x0, y0, r):
  // """
  // Calculates the distance between a point (x, y) and the border of a circle with center (x0, y0) and radius r.
  // """
  // dx = x - x0
  // dy = y - y0
  // distance = math.sqrt(dx ** 2 + dy ** 2) - r
  // return distance
}
