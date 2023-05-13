use std::collections::VecDeque;

pub fn vec_sample(){
    println!("\n***\n\nVectors:");

    let mut numbers = Vec::from([-1, 0, 1, 100]);
    assert_eq!(numbers.len(), 4);
    assert_eq!(numbers, vec![-1, 0, 1, 100]);
    let x = numbers.get(3);
    assert!(x.is_some());
    assert_eq!(*x.unwrap(), 100);

    numbers[3] = 200;    
    assert_eq!(numbers, vec![-1, 0, 1, 200]);

    let mut days: Vec<&str> = Vec::new();
    days.push("Monday");
    days.push("Tuesday");
    days.push("Wednesday");
    assert_eq!(days.len(), 3);

    let mut vec1 = vec![1, 2, 3];
    println!("Sample vector1 {:?}", vec1);

    vec1.push(4);
    vec1.push(5);
    assert_eq!(vec1, vec![1,2,3,4,5]);
    println!("After Push(4), Push(5) {:?}", vec1);

    vec1.pop();
    assert_eq!(vec1, vec![1,2,3,4]);
    println!("After Pop() {:?}", vec1);

    for item in vec1.iter_mut() {
        *item += 10;
    }
    assert_eq!(vec1, vec![11,12,13,14]);
    println!("After +10 {:?}", vec1);

    vec1.remove(2);
    assert_eq!(vec1, vec![11,12,14]);
    println!("After remove(2) {:?}", vec1);

    vec1.insert(1,77);
    assert_eq!(vec1, vec![11,77,12,14]);
    println!("After insert(1,77) {:?}", vec1);

    vec1.reverse();
    println!("After reverse {:?}", vec1);

    vec1.sort();
    println!("After sort {:?}", vec1);

    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut vec2: Vec<_> = (0..10).map(|_| rng.gen::<u8>()).collect();
    println!("Random numbers{:?}", vec2);

    vec2.sort_by(|a, b| b.cmp(a));
    println!("After reverse sort {:?}", vec2);

}

pub fn vec_deque_sample() {

    let mut vec_deque1 = VecDeque::from([3,1,2]);
    println!("VecDeque sample1 {:?}", vec_deque1);

    vec_deque1.push_front(0);
    println!("VecDeque after push_front {:?}", vec_deque1);

    vec_deque1.push_back(4);
    println!("VecDeque after push_back {:?}", vec_deque1);

    vec_deque1.pop_front();
    println!("VecDeque after pop_front {:?}", vec_deque1);

    vec_deque1.pop_back();
    println!("VecDeque after pop_back {:?}", vec_deque1);

    vec_deque1.as_mut_slices().0.sort();
    println!("VecDeque after sort {:?}", vec_deque1);

    vec_deque1.as_mut_slices().0.sort_by(|a, b| b.cmp(a));
    println!("VecDeque after reverse sort {:?}", vec_deque1);
}