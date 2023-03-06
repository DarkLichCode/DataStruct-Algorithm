mod LinearList;
use LinearList::LinkedListLib::LinkedList;

fn main() {
	let mut link = LinkedList::New();
    for i in 0..2 {
        link.pushBack(i);
    }

    let mut tempHead = link.head.as_ref();
    while let Some(node) = tempHead.take() {
        println!("{}", node.data);
        tempHead = node.next.as_ref();
    }

    // link.reverse();

    // link.removeHead();
    link.removeBack();
    link.removeBack();

    tempHead = link.head.as_ref();
    while let Some(node) = tempHead.take() {
        println!("{}", node.data);
        tempHead = node.next.as_ref();
    }
}
