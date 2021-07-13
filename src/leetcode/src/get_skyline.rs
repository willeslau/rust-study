struct SlowSolution;

/// TODO: There is a much simpler and elegant solution. Try again later.
impl SlowSolution {
	pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
		let mut buildings = buildings;
		buildings.sort_by(|a, b| {
			let r = a[0].cmp(&b[0]);
			if r.is_eq() {
				return a[1].cmp(&b[1]);
			}
			r
		});

		println!("{:?}", buildings);

		let mut stack = vec![];
		for i in 0..buildings.len() {
			Self::merge(&mut stack, &buildings[i]);
			println!("stack: {:?}, i: {}", stack, i);
		}

		println!("{:?}", stack);

		let mut result = vec![];
		result.push(vec![stack[0][0], stack[0][2]]);
		for i in 1..stack.len() {
			if stack[i][0] != stack[i-1][1] {
				result.push(vec![stack[i-1][1], 0]);
			}
			result.push(vec![stack[i][0], stack[i][2]]);
		}

		result.push(vec![stack[stack.len()-1][1], 0]);
		result
    }

	fn merge(stack: &mut Vec<Vec<i32>>, interval: &Vec<i32>) {
		loop {
			if stack.is_empty() {
				stack.push(interval.clone());
				return;
			}
			let n = stack.pop().unwrap();
			if Self::is_before(interval, &n) {
				Self::merge(stack, interval);
				Self::merge(stack, &n);
				return;
			}
			let (is_intersect, intervals) = Self::intersect(&n, interval);
			if !is_intersect {
				stack.push(n);
				stack.push(interval.clone());
			} else {
				for i in intervals {
					Self::merge(stack, &i);
				}
			}
			return;
		}
	}

	fn is_before(h: &Vec<i32>, w: &Vec<i32>) -> bool {
		match h[0].cmp(&w[0]) {
			std::cmp::Ordering::Less => true,
			std::cmp::Ordering::Equal => h[1] < w[1],
			_ => false
		}
	}

	fn intersect(h: &Vec<i32>, w: &Vec<i32>) -> (bool, Vec<Vec<i32>>) {
		let a;
		let b;
		match h[0].cmp(&w[0]) {
			std::cmp::Ordering::Greater => {
				a = w;
				b = h;
			},
			std::cmp::Ordering::Equal => {
				if h[1] > w[1] {
					a = w;
					b = h;
				} else {
					a = h;
					b = w;
				}
			},
			_ => {
				a = h;
				b = w;
			}
		}

		// println!("a: {:?}, b: {:?}", a, b);
		if a[1] < b[0] {
			return (false, vec![a.clone(), b.clone()]);
		}

		if a[1] == b[0] {
			if a[2] == b[2] {
				return (true, vec![vec![a[0], b[1], a[2]]]);
			}
			return (false, vec![a.clone(), b.clone()]);
		}

		if a[2] == b[2] {
			return (true, vec![vec![a[0], b[1], a[2]]]);
		}

		let mut result = vec![];

		if a[0] == b[0] {
			return match a[1].cmp(&b[1]) {
				std::cmp::Ordering::Greater => {
					if a[2] >= b[2] {
						(true, vec![vec![a[0], a[1], a[2]]])
					} else {
						(true, vec![
							vec![b[0], b[1], b[2]],
							vec![b[1], a[1], a[2]],
						])
					}
				},
				std::cmp::Ordering::Equal => {
					(true, vec![vec![a[0], a[1], std::cmp::max(a[2], b[2])]])
				},
				std::cmp::Ordering::Less => {
					match a[2].cmp(&b[2]) {
						std::cmp::Ordering::Greater => {
							result.push(vec![a[0], a[1], a[2]]);
							result.push(vec![a[1], b[1], b[2]]);
						},
						_ => {
							result.push(vec![a[0], b[1], b[2]]);
						}
					}
					(true, result)
				}
			}
		}

		return match a[1].cmp(&b[1]) {
			std::cmp::Ordering::Greater => {
				if a[2] >= b[2] {
					(true, vec![vec![a[0], a[1], a[2]]])
				} else {
					(true, vec![
						vec![a[0], b[0], a[2]],
						vec![b[0], b[1], b[2]],
						vec![b[1], a[1], a[2]],
					])
				}
			},
			std::cmp::Ordering::Equal => {
				if a[2] >= b[2] {
					(true, vec![vec![a[0], a[1], a[2]]])
				} else {
					(true, vec![
						vec![a[0], b[0], a[2]],
						vec![b[0], b[1], b[2]],
					])
				}
			},
			std::cmp::Ordering::Less => {
				match a[2].cmp(&b[2]) {
					std::cmp::Ordering::Greater => {
						(true, vec![
							vec![a[0], a[1], a[2]],
							vec![a[1], b[1], b[2]],
						])
					},
					_ => {
						(true, vec![
							vec![a[0], b[0], a[2]],
							vec![b[0], b[1], b[2]],
						])
					}
				}
			}
		}
	}
}



#[cfg(test)]
mod tests {
    use crate::get_skyline::SlowSolution;

    #[test]
    fn test_intersect_1() {
		let v = SlowSolution::intersect(&vec![3, 8, 12], &vec![5, 12, 13]);
    	assert_eq!(v, (true, vec![vec![3,5,12], vec![5,12,13]]));

		let v = SlowSolution::intersect(&vec![5, 12, 13], &vec![3, 8, 12]);
		assert_eq!(v, (true, vec![vec![3,5,12], vec![5,12,13]]));
    }

	#[test]
	fn test_intersect_2() {
		let v = SlowSolution::intersect(&vec![7, 8, 12], &vec![5, 12, 13]);
		assert_eq!(v, (true, vec![vec![5,12,13]]));
	}

	#[test]
	fn test_intersect_3() {
		let v = SlowSolution::intersect(&vec![3, 8, 12], &vec![3, 12, 13]);
		assert_eq!(v, (true, vec![vec![3,12,13]]));

		let v = SlowSolution::intersect(&vec![5, 12, 13], &vec![3, 8, 12]);
		assert_eq!(v, (true, vec![vec![3,5,12], vec![5,12,13]]));

		let v = SlowSolution::intersect(&vec![3, 8, 12], &vec![5, 12, 13]);
		assert_eq!(v, (true, vec![vec![3,5,12], vec![5,12,13]]));
	}

	#[test]
	fn test_intersect_4() {
		let v = SlowSolution::intersect(&vec![1, 2, 3], &vec![2, 3, 3]);
		assert_eq!(v, (true, vec![vec![1,3,3]]));
	}

	#[test]
	fn test_case_1() {
		let v = vec![
			vec![3,7,15],
			vec![3,6,15],
			vec![5,12,13],
			vec![15,20,10],
			vec![3,8,12],
			vec![19,24,8],
			vec![2,9,10]
		];

		let result = vec![
			vec![2,10],
			vec![3,15],
			vec![7,13],
			vec![12,0],
			vec![15,10],
			vec![20,8],
			vec![24,0]
		];
		assert_eq!(SlowSolution::get_skyline(v), result);
	}


	#[test]
	fn test_case_2() {
		let v = vec![vec![0,2,3],vec![2,5,3]];
		let result = vec![[0,3],[5,0]];
		assert_eq!(SlowSolution::get_skyline(v), result);
	}

	#[test]
	fn test_case_3() {
		let v = vec![[1,38,219],[2,19,228],[2,64,106],[3,80,65],[4,46,225]];
		let v = v.iter().map(|a| a.to_vec()).collect();
		let result = vec![[1,219],[2,228],[19,225],[46,106],[64,65],[80,0]];
		assert_eq!(SlowSolution::get_skyline(v), result);
	}

	#[test]
	fn test_case_4() {
		let v = vec![[1,38,219],[2,19,228],[2,64,106],[3,80,65],[3,84,8],[4,12,8],[4,25,14],[4,46,225],[4,67,187],[5,36,118],[5,48,211],[5,55,97],[6,42,92],[6,56,188],[7,37,42],[7,49,78],[7,84,163],[8,44,212],[9,42,125],[9,85,200],[9,100,74],[10,13,58],[11,30,179],[12,32,215],[12,33,161],[12,61,198],[13,38,48],[13,65,222],[14,22,1],[15,70,222],[16,19,196],[16,24,142],[16,25,176],[16,57,114],[18,45,1],[19,79,149],[20,33,53],[21,29,41],[23,77,43],[24,41,75],[24,94,20],[27,63,2],[31,69,58],[31,88,123],[31,88,146],[33,61,27],[35,62,190],[35,81,116],[37,97,81],[38,78,99],[39,51,125],[39,98,144],[40,95,4],[45,89,229],[47,49,10],[47,99,152],[48,67,69],[48,72,1],[49,73,204],[49,77,117],[50,61,174],[50,76,147],[52,64,4],[52,89,84],[54,70,201],[57,76,47],[58,61,215],[58,98,57],[61,95,190],[66,71,34],[66,99,53],[67,74,9],[68,97,175],[70,88,131],[74,77,155],[74,99,145],[76,88,26],[82,87,40],[83,84,132],[88,99,99]];
		let v = v.iter().map(|a| a.to_vec()).collect();
		let result = vec![[1,219],[2,228],[19,225],[45,229],[89,190],[95,175],[97,152],[99,74],[100,0]];
		assert_eq!(SlowSolution::get_skyline(v), result);
	}

	#[test]
	fn test_case_5() {
		let v = vec![[0,3,3],[1,5,3],[2,4,3],[3,7,3]];
		let v = v.iter().map(|a| a.to_vec()).collect();
		let result = vec![[0,3],[7,0]];
		assert_eq!(SlowSolution::get_skyline(v), result);
	}

	#[test]
	fn test_case_6() {
		let v = vec![[1,2,1],[1,2,2],[1,2,3],[2,3,1],[2,3,2],[2,3,3]];
		let v = v.iter().map(|a| a.to_vec()).collect();
		let result = vec![[1, 3], [3, 0]];
		assert_eq!(SlowSolution::get_skyline(v), result);
	}
}