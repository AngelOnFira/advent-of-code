use std::collections::{VecDeque, HashMap};

fn main() {
    let num_players = 447;
    let highest_marble = 7151000;

    let mut circle: VecDeque<i32> = VecDeque::with_capacity(highest_marble);
    let mut scores: HashMap<i32, i32> = HashMap::new();
    
    circle.push_back(0);
    circle.push_back(1);
    circle.push_back(2);
    circle.push_back(3);
    circle.push_back(4);
    // let mut location = circle.iter();
    // location = location.next();
    // println!("{}", location.next().unwrap());
    // for i in 1..(highest_marble + 1) {

    // }
    // buf.push_back(3);
    // buf.push_back(4);
    // buf.push_back(5);
    // assert_eq!(buf.get(1), Some(&4));
}



// from collections import defaultdict

// class Node:
//     def __init__(self, dataval=None):
//         self.data = dataval
//         self.next = None
//         self.prev = None

// class DLinkedList:
//     def __init__(self):
//         self.head = None

// list1 = DLinkedList()
// list1.head = Node(0)

// num_players = 447
// highest_marble = 7151000

// player_dict = defaultdict(int)

// player_turn = 0
// current_pos = 1

// location = list1.head
// location.next = location
// location.prev = location

// for marble in range(1, highest_marble + 1):
    
//     if marble % 23 == 0:
//         location = location.prev.prev.prev.prev.prev.prev.prev.prev
//         popped_marble = location.data
//         location = location.prev
//         location.next = location.next.next
        
//         player_dict[player_turn] += marble + popped_marble
        
//         location = location.next.next
//     else:
//         next_node = location.next
//         this_node = Node(marble)
        
//         this_node.prev = location
//         this_node.next = next_node
        
//         location.next = this_node
        
//         this_node.next = next_node
//         this_node.prev = location
        
//         next_node.prev = this_node
        
//         location = location.next.next
        
//     player_turn = (player_turn + 1) % num_players

// print max(player_dict.values())