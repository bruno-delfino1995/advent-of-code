use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::{Rc, Weak};

use crate::prelude::*;
use INode::*;

type SharedINode = Rc<RefCell<INode>>;
type LinkedINode = Weak<RefCell<INode>>;

#[derive(Clone)]
enum INode {
	Directory {
		name: String,
		parent: Option<LinkedINode>,
		children: HashMap<String, SharedINode>,
	},
	File {
		name: String,
		size: usize,
		parent: LinkedINode,
	},
}

impl INode {
	fn dir(name: String, parent: Option<&SharedINode>) -> SharedINode {
		let dir = Directory {
			name,
			parent: parent.map(Rc::downgrade),
			children: HashMap::new(),
		};

		Rc::new(RefCell::new(dir))
	}

	fn file(name: String, size: usize, parent: &SharedINode) -> SharedINode {
		let file = File {
			name,
			size,
			parent: Rc::downgrade(parent),
		};

		Rc::new(RefCell::new(file))
	}

	fn is_dir(&self) -> bool {
		matches!(self, Directory { .. })
	}

	fn add(&mut self, child: &SharedINode) {
		match self {
			File { .. } => (),
			Directory { children, .. } => {
				children.insert(child.borrow().name().to_string(), Rc::clone(child));
			}
		}
	}

	fn parent(&self) -> Option<SharedINode> {
		match self {
			File { parent, .. } => Weak::upgrade(parent),
			Directory { parent, .. } => parent.as_ref().and_then(Weak::upgrade),
		}
	}

	fn children(&self) -> Option<&HashMap<String, SharedINode>> {
		match self {
			Directory { children, .. } => Some(children),
			File { .. } => None,
		}
	}

	fn name(&self) -> &str {
		match self {
			Directory { name, .. } => name,
			File { name, .. } => name,
		}
	}

	fn size(&self) -> usize {
		match self {
			Directory { children, .. } => children
				.iter()
				.fold(0, |acc, (_, el)| acc + el.borrow().size()),
			File { size, .. } => *size,
		}
	}

	fn tree(&self, padding: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let prefix = "  ".repeat(padding);

		match self {
			Directory { name, children, .. } => {
				writeln!(f, "{}- {} (dir) (child={})", prefix, name, children.len())?;
				children
					.iter()
					.try_for_each(|(_, child)| child.borrow().tree(padding + 1, f))
			}
			File { name, .. } => writeln!(f, "{}- {} (file, size={})", prefix, name, self.size()),
		}
	}
}

impl fmt::Debug for INode {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.tree(0, f)
	}
}

struct INodeWrapper(SharedINode);

impl IntoIterator for INodeWrapper {
	type Item = SharedINode;

	type IntoIter = INodeIter;

	fn into_iter(self) -> Self::IntoIter {
		INodeIter {
			root: Some(self.0),
			children: vec![],
			parent: None,
		}
	}
}

#[derive(Default)]
struct INodeIter {
	root: Option<SharedINode>,
	children: Vec<SharedINode>,
	parent: Option<Box<INodeIter>>,
}

impl Iterator for INodeIter {
	type Item = SharedINode;

	fn next(&mut self) -> Option<Self::Item> {
		let has_root = self.root.is_some();
		let has_children = !self.children.is_empty();
		let has_parent = self.parent.is_some();

		if !has_root && !has_children && has_parent {
			let parent = self.parent.take().unwrap();
			*self = *parent;

			return self.next();
		}

		if has_root {
			let root = self.root.take().unwrap();
			self.children = root
				.borrow()
				.children()
				.map(|c| c.values().cloned().collect())
				.unwrap_or_default();

			return Some(root);
		}

		if !has_children {
			return None;
		}

		let curr = self.children.pop().unwrap();
		let check = curr.borrow().to_owned();
		match check {
			File { .. } => Some(curr),
			Directory { .. } => {
				*self = INodeIter {
					root: Some(curr),
					children: vec![],
					parent: Some(Box::new(std::mem::take(self))),
				};

				self.next()
			}
		}
	}
}

struct Runtime {
	root: Option<SharedINode>,
	cwd: Option<SharedINode>,
}

impl Runtime {
	fn new() -> Self {
		Self {
			root: None,
			cwd: None,
		}
	}

	fn root(&self) -> Option<SharedINode> {
		self.root.as_ref().map(Rc::clone)
	}

	fn cwd(&self) -> Option<SharedINode> {
		self.cwd.as_ref().map(Rc::clone)
	}

	fn enter(&mut self, dir: &SharedINode) {
		match *dir.borrow() {
			File { .. } => (),
			Directory { .. } => {
				if self.root.is_none() {
					self.root = Some(Rc::clone(dir))
				}

				self.cwd = Some(Rc::clone(dir))
			}
		}
	}

	fn up(&mut self) {
		match &self.cwd.as_ref().and_then(|cwd| cwd.borrow().parent()) {
			None => (),
			Some(cwd) => {
				self.cwd = Some(Rc::clone(cwd));
			}
		}
	}
}

struct Shell {
	runtime: Runtime,
}

impl Shell {
	fn new() -> Self {
		Shell {
			runtime: Runtime::new(),
		}
	}

	fn parse(self, line: &str) -> Self {
		if line.starts_with("$ cd") {
			return self.cd(line.strip_prefix("$ cd ").unwrap());
		}

		if line.eq("$ ls") {
			return self;
		}

		self.ls(line)
	}

	fn cd(mut self, dir: &str) -> Self {
		if dir.eq("..") {
			self.runtime.up();

			return self;
		}

		let entry = self.runtime.cwd().and_then(|cwd| {
			cwd.borrow()
				.children()
				.and_then(|children| children.get(dir).map(|dir| self.runtime.enter(dir)))
		});

		match entry {
			Some(_) => (),
			None => {
				let cwd = self.runtime.cwd();
				let dir = INode::dir(String::from(dir), cwd.as_ref());

				self.runtime.enter(&dir)
			}
		}

		self
	}

	fn ls(self, desc: &str) -> Self {
		let cwd = self.runtime.cwd().unwrap();

		let node = if desc.starts_with("dir") {
			let name = String::from(desc.strip_prefix("dir ").unwrap());

			INode::dir(name, Some(&cwd))
		} else {
			let (size, name) = desc.split_once(' ').unwrap();
			let size = size.parse::<usize>().unwrap();
			let name = String::from(name);

			INode::file(name, size, &cwd)
		};

		cwd.borrow_mut().add(&node);

		self
	}

	fn close(self) -> Runtime {
		self.runtime
	}
}

pub fn basic(input: Input) -> String {
	let shell = lines(input).fold(Shell::new(), |shell, line| shell.parse(&line));

	let root = shell.close().root().unwrap();

	INodeWrapper(root)
		.into_iter()
		.filter(|p| p.borrow().is_dir())
		.map(|d| d.borrow().size())
		.filter(|&s| s < 100000)
		.sum::<usize>()
		.to_string()
}

pub fn complex(input: Input) -> String {
	const FS_SIZE: usize = 70000000;
	const UPDATE_SIZE: usize = 30000000;

	let shell = lines(input).fold(Shell::new(), |shell, line| shell.parse(&line));
	let root = shell.close().root().unwrap();
	let unused = FS_SIZE - root.borrow().size();
	let to_delete = UPDATE_SIZE - unused;

	INodeWrapper(root)
		.into_iter()
		.filter(|p| p.borrow().is_dir())
		.map(|d| d.borrow().size())
		.filter(|&s| s > to_delete)
		.min()
		.unwrap_or_default()
		.to_string()
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::input;

	#[test]
	fn first_example() {
		let input = input!(
			r#"
			$ cd /
			$ ls
			dir a
			14848514 b.txt
			8504156 c.dat
			dir d
			$ cd a
			$ ls
			dir e
			29116 f
			2557 g
			62596 h.lst
			$ cd e
			$ ls
			584 i
			$ cd ..
			$ cd ..
			$ cd d
			$ ls
			4060174 j
			8033020 d.log
			5626152 d.ext
			7214296 k
		"#
		);

		assert_eq!(basic(input), "95437")
	}

	#[test]
	fn second_example() {
		let input = input!(
			r#"
			$ cd /
			$ ls
			dir a
			14848514 b.txt
			8504156 c.dat
			dir d
			$ cd a
			$ ls
			dir e
			29116 f
			2557 g
			62596 h.lst
			$ cd e
			$ ls
			584 i
			$ cd ..
			$ cd ..
			$ cd d
			$ ls
			4060174 j
			8033020 d.log
			5626152 d.ext
			7214296 k
		"#
		);

		assert_eq!(complex(input), "24933642")
	}
}
