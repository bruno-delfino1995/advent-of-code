use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::fmt;
use std::rc::{Rc, Weak};

use crate::prelude::*;
use INode::*;

#[derive(Clone)]
enum INode {
	Directory {
		name: String,
		parent: Option<Weak<INode>>,
		children: RefCell<HashMap<String, Rc<INode>>>,
	},
	File {
		name: String,
		size: usize,
		parent: Weak<INode>,
	},
}

impl INode {
	fn dir(name: String, parent: Option<&Rc<INode>>) -> Self {
		Directory {
			name,
			parent: parent.map(Rc::downgrade),
			children: RefCell::new(HashMap::new()),
		}
	}

	fn file(name: String, size: usize, parent: &Rc<INode>) -> Self {
		File {
			name,
			size,
			parent: Rc::downgrade(parent),
		}
	}

	fn is_dir(&self) -> bool {
		matches!(self, Directory { .. })
	}

	fn add(&self, child: &Rc<INode>) {
		match self {
			File { .. } => (),
			Directory { children, .. } => {
				children
					.borrow_mut()
					.insert(child.name().to_string(), Rc::clone(child));
			}
		}
	}

	fn parent(&self) -> Option<Rc<INode>> {
		match self {
			File { parent, .. } => Weak::upgrade(parent),
			Directory { parent, .. } => parent.as_ref().and_then(Weak::upgrade),
		}
	}

	fn children(&self) -> Option<Ref<HashMap<String, Rc<INode>>>> {
		match self {
			Directory { children, .. } => Some(children.borrow()),
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
				.borrow()
				.iter()
				.fold(0, |acc, (_, el)| acc + el.size()),
			File { size, .. } => *size,
		}
	}

	fn tree(&self, padding: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let prefix = "  ".repeat(padding);

		match self {
			Directory { name, children, .. } => {
				writeln!(
					f,
					"{}- {} (dir) (child={})",
					prefix,
					name,
					children.borrow().len()
				)?;
				children
					.borrow()
					.iter()
					.try_for_each(|(_, child)| child.tree(padding + 1, f))
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

struct INodeWrapper(Rc<INode>);

impl IntoIterator for INodeWrapper {
	type Item = Rc<INode>;

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
	root: Option<Rc<INode>>,
	children: Vec<Rc<INode>>,
	parent: Option<Box<INodeIter>>,
}

impl Iterator for INodeIter {
	type Item = Rc<INode>;

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
				.children()
				.map(|c| c.values().cloned().collect())
				.unwrap_or_default();

			return Some(root);
		}

		if !has_children {
			return None;
		}

		let curr = self.children.pop().unwrap();
		match *curr {
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
	root: Option<Rc<INode>>,
	cwd: Option<Rc<INode>>,
}

impl Runtime {
	fn new() -> Self {
		Self {
			root: None,
			cwd: None,
		}
	}

	fn root(&self) -> Option<Rc<INode>> {
		self.root.as_ref().map(Rc::clone)
	}

	fn cwd(&self) -> Option<Rc<INode>> {
		self.cwd.as_ref().map(Rc::clone)
	}

	fn enter(&mut self, dir: &Rc<INode>) {
		match **dir {
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
		match &self.cwd.as_ref().and_then(|cwd| cwd.parent()) {
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
			cwd.children()
				.and_then(|children| children.get(dir).map(|dir| self.runtime.enter(dir)))
		});

		match entry {
			Some(_) => (),
			None => {
				let cwd = self.runtime.cwd();
				let dir = Rc::new(INode::dir(String::from(dir), cwd.as_ref()));

				self.runtime.enter(&dir)
			}
		}

		self
	}

	fn ls(self, desc: &str) -> Self {
		let cwd = self.runtime.cwd().unwrap();

		let node = if desc.starts_with("dir") {
			let name = String::from(desc.strip_prefix("dir ").unwrap());

			Rc::new(INode::dir(name, Some(&cwd)))
		} else {
			let (size, name) = desc.split_once(' ').unwrap();
			let size = size.parse::<usize>().unwrap();
			let name = String::from(name);

			Rc::new(INode::file(name, size, &cwd))
		};

		cwd.add(&node);

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
		.filter(|p| p.is_dir())
		.map(|d| d.size())
		.filter(|&s| s < 100000)
		.sum::<usize>()
		.to_string()
}

pub fn complex(_input: Input) -> String {
	String::from("unimplemented")
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
			3
		"#
		);

		assert_eq!(complex(input), "2")
	}
}
