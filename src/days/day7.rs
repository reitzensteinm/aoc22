use std::fs::read_to_string;

enum INode {
    File(String, Option<usize>, u32),
    Directory(String, Option<usize>, Vec<usize>),
}

struct FileSystem {
    nodes: Vec<INode>,
}

impl FileSystem {
    fn add_directory(&mut self, name: &str, parent: Option<usize>) -> usize {
        let node = self.nodes.len();
        self.nodes
            .push(INode::Directory(name.to_string(), parent, vec![]));
        if let Some(p) = parent {
            if let INode::Directory(_, _, children) = &mut self.nodes[p] {
                children.push(node)
            }
        }
        node
    }
    fn add_file(&mut self, name: &str, size: u32, parent: Option<usize>) -> usize {
        let node = self.nodes.len();
        self.nodes.push(INode::File(name.to_string(), parent, size));
        if let Some(p) = parent {
            if let INode::Directory(_, _, children) = &mut self.nodes[p] {
                children.push(node)
            }
        }
        node
    }

    fn directory_size(&self, cached: &mut Vec<Option<u32>>, node: usize) -> u32 {
        match (&self.nodes[node], cached[node]) {
            (INode::File(_, _, size), _) => *size,
            (_, Some(s)) => s,
            (INode::Directory(_, _, children), _) => {
                let mut size = 0;
                for c in children {
                    size += self.directory_size(cached, *c);
                }
                cached[node] = Some(size);
                size
            }
        }
    }

    fn scan_directories(&self) -> (u32, u32) {
        let mut sizes: Vec<Option<u32>> = vec![None; self.nodes.len()];
        let mut sum_of_moderate = 0;
        let mut smallest_acceptable: Option<u32> = None;

        for n in 0..self.nodes.len() {
            if let INode::Directory(_, _, _) = self.nodes[n] {
                let size = self.directory_size(&mut sizes, n);
                if size <= 100_000 {
                    sum_of_moderate += size;
                }

                let free_space = 70_000_000 - sizes[0].unwrap();
                if free_space + size >= 30_000_000 {
                    if let Some(s) = smallest_acceptable {
                        smallest_acceptable = Some(s.min(size));
                    } else {
                        smallest_acceptable = Some(size);
                    }
                }
            }
        }

        (sum_of_moderate, smallest_acceptable.unwrap())
    }
}

pub fn day_7() -> (String, String) {
    let f = read_to_string("input/day7.txt").unwrap();

    let mut fs = FileSystem { nodes: vec![] };
    let mut nodes = vec![];

    for l in f.lines() {
        if l.starts_with("$ cd") {
            let (_, dn) = l.split_at(5);

            let node = if dn == "/" {
                Some(fs.add_directory("", None))
            } else if dn == ".." {
                nodes.pop();
                None
            } else {
                Some(fs.add_directory("dn", Some(*nodes.last().unwrap())))
            };

            if let Some(n) = node {
                nodes.push(n);
            }
        } else if l.starts_with("$") {
        } else if l.starts_with("dir") {
        } else if l != "" {
            let (size, name) = l.split_once(" ").unwrap();
            let sizei = str::parse::<u32>(size).unwrap();
            fs.add_file(name, sizei, Some(*nodes.last().unwrap()));
        }
    }

    let (a, b) = fs.scan_directories();
    (format!("{}", a), format!("{}", b))
}
