mod LinearList;
use LinearList::LinkedListLib::LinkedList;

fn main() {
	let mut link = LinkedList::New();
    for i in 0..10 {
        link.pushBack(i);
    }

    let mut tempHead = link.head.as_ref();
    while let Some(node) = tempHead.take() {
        println!("{}", node.data);
        tempHead = node.next.as_ref();
    }

    // link.reverse();
    link.removeHead();
    tempHead = link.head.as_ref();
    while let Some(node) = tempHead.take() {
        println!("{}", node.data);
        tempHead = node.next.as_ref();
    }
}
