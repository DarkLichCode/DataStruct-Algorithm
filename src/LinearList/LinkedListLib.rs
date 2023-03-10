use std::mem::take;

pub struct ListNode<T>{
	pub(crate) data: T,
	pub(crate) next: Option<Box<ListNode<T>>>
}

pub struct LinkedList<T>{
	pub(crate) head: Option<Box<ListNode<T>>>,
	pub(crate) size: usize
}

impl<T> ListNode<T> {
	pub fn New(data: T) -> Option<Box<ListNode<T>>> {
		let mut node: Option<Box<ListNode<T>>> = Some(Box::new(ListNode{
			data: data,
			next: None
		}));
		return node;
	}
}

impl<T> LinkedList<T> {
	pub fn New() -> Self {
		let link = LinkedList {
			head: None,
			size: 0
		};
		return link;
	}

	// 头插法
	pub fn pushHead(&mut self, data: T) {
		if self.head.is_none() {
			self.head = ListNode::New(data);
		} else {
			let mut tempNode = ListNode::New(data);
			if let (Some(mut node), Some(mut head)) = (tempNode, self.head.take()) {
				node.next = Some(head);
				self.head = Some(node);
			}
		}
		self.size += 1;
	}

	// 删除头部节点
	pub fn removeHead(&mut self) -> bool {
		if self.size == 0 {
			return false;
		}

		let tempNode = self.head.as_mut();
		if let Some(node) = tempNode {
			self.head = node.next.take();
		}
		self.size -= 1;
		return true;
	}

	// 尾插法
	pub fn pushBack(&mut self, data: T) {
		if self.head.is_none() {
			self.head = ListNode::New(data);
		} else {
			let mut newNode = ListNode::New(data);
			let mut tempNode = self.head.as_mut();
			while let Some(node) = tempNode {
				if node.next.is_none() {
					tempNode = Some(node);
					break;
				}
				tempNode = node.next.as_mut();
			}

			if let (Some(mut node), Some(mut backNode)) = (newNode, tempNode) {
				backNode.next = Some(node);
			}
		}
		self.size += 1;
	}

	// 删除尾部节点
	pub fn removeBack(&mut self) -> bool {
		if self.size == 0 {
			return false;
		}

		if self.size == 1 {
			self.removeHead();
			return true;
		}
		let mut tempNode = self.head.as_mut();
		while let Some(mut curNode) = tempNode.take() {
			let mut nextNode = curNode.next.take().unwrap();
			if nextNode.next.is_none() {
				curNode.next = None;
				break;
			} else {
				curNode.next = Some(nextNode);
			}
			tempNode = curNode.next.as_mut();
		}

		self.size -= 1;
		return true;
	}

	// 判断链表是否为空
	pub fn isEmpty(&self) -> bool {
		return self.size == 0;
	}

	// 获取链表长度
	pub fn Length(&self) -> usize {
		return self.size;
	}

	// 翻转链表
	pub fn reverse(&mut self) {
		fn reverseNode<T>(preNode: Option<Box<ListNode<T>>>, curNode: Option<Box<ListNode<T>>>) -> Option<Box<ListNode<T>>> {
			if let Some(mut node) = curNode{
				let nextNode = node.next.take();
				node.next = preNode;
				return reverseNode(Some(node), nextNode);
			}
			return preNode;
		}
		self.head = reverseNode(None, self.head.take());
	}


}

