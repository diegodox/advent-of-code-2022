use std::{collections::HashMap, io::BufRead};

fn main() {
    dbg!(part1());
    dbg!(part2());
}

struct DirPoolSized<'p> {
    _pool: &'p [Dir],
    dir_sizes: Vec<usize>,
}

struct DirPool {
    pool: Vec<Dir>,
    cwd_id: usize,
}

impl std::ops::Index<usize> for DirPool {
    type Output = Dir;

    fn index(&self, index: usize) -> &Self::Output {
        self.pool.index(index)
    }
}

impl std::ops::IndexMut<usize> for DirPool {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.pool.index_mut(index)
    }
}

impl FromIterator<String> for DirPool {
    fn from_iter<T: IntoIterator<Item = String>>(lines: T) -> Self {
        let mut dir_pool = DirPool::new();

        let mut lines = lines.into_iter();
        lines.next();
        for line in lines {
            let mut words = line.split_whitespace();
            match words.next().unwrap() {
                "$" => {
                    match words.next().unwrap() {
                        "cd" => {
                            match words.next().unwrap() {
                                ".." => dir_pool.cd_parent(),
                                dir_name => dir_pool.cd_child(dir_name),
                            };
                        }
                        "ls" => { /* do nothing */ }
                        _ => unreachable!(),
                    };
                }
                "dir" => {
                    let dir_name = words.next().unwrap().to_string();
                    // dbg!(&dir_name);
                    dir_pool.mkdir(dir_name);
                }
                size => dir_pool.add_file(size.parse::<usize>().unwrap()),
            }
        }

        dir_pool
    }
}

impl DirPool {
    fn new() -> Self {
        Self {
            pool: vec![Dir {
                name: "/".to_string(),
                parent_dir_id: None,
                child_dir_ids: HashMap::new(),
                size: 0,
            }],
            cwd_id: 0,
        }
    }

    fn cd_parent(&mut self) {
        self.cwd_id = self.current_dir().parent_dir_id.unwrap()
    }

    fn cd_child(&mut self, name: &str) {
        self.cwd_id = self.current_dir().child_dir_ids[name]
    }

    fn mkdir(&mut self, name: String) -> usize {
        self.pool.push(Dir::new_child(name.clone(), self.cwd_id));
        let new_id = self.pool.len() - 1;
        self.current_dir_mut().child_dir_ids.insert(name, new_id);
        new_id
    }

    fn add_file(&mut self, size: usize) {
        self.current_dir_mut().size += size
    }

    fn current_dir(&self) -> &Dir {
        &self.pool[self.cwd_id]
    }

    fn current_dir_mut(&mut self) -> &mut Dir {
        &mut self.pool[self.cwd_id]
    }

    fn dir_sizes(&self) -> DirPoolSized<'_> {
        let mut ret = DirPoolSized {
            _pool: &self.pool,
            dir_sizes: vec![0; self.pool.len()],
        };

        let mut que = std::collections::VecDeque::new();

        let mut working_que = std::collections::VecDeque::new();
        working_que.push_back(0);
        while let Some(cursor) = working_que.pop_front() {
            que.push_back(cursor);
            self[cursor]
                .child_dir_ids
                .values()
                .for_each(|id| working_que.push_back(*id));
        }

        while let Some(x) = que.pop_back() {
            ret.dir_sizes[x] = self[x].size
                + self[x]
                    .child_dir_ids
                    .values()
                    .map(|&id| ret.dir_sizes[id])
                    .sum::<usize>();
        }

        ret
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Dir {
    name: String,
    parent_dir_id: Option<usize>,
    child_dir_ids: HashMap<String, usize>,
    size: usize,
}

impl Dir {
    fn new_child(name: String, parent_dir_id: usize) -> Self {
        Self {
            name,
            parent_dir_id: Some(parent_dir_id),
            child_dir_ids: Default::default(),
            size: 0,
        }
    }
}

fn part1() -> usize {
    let dir_pool = input().lines().map(|x| x.unwrap()).collect::<DirPool>();

    dir_pool
        .dir_sizes()
        .dir_sizes
        .into_iter()
        .filter(|size| size <= &100000)
        .sum()
}

fn part2() -> usize {
    const DISK_SPACE: usize = 70000000;
    const SPACE_NEEDED: usize = 30000000;

    let dir_pool = input().lines().map(|x| x.unwrap()).collect::<DirPool>();
    let x = dir_pool.dir_sizes();
    let used: usize = x.dir_sizes[0];
    let delete = used + SPACE_NEEDED - DISK_SPACE;
    x.dir_sizes
        .into_iter()
        .filter(|&size| size >= delete)
        .min()
        .unwrap()
}

fn input() -> std::io::BufReader<std::fs::File> {
    let mut p = std::path::PathBuf::from(std::env!("CARGO_MANIFEST_DIR"));
    p.push("src");
    p.push("bin");
    p.push(module_path!().split("::").last().unwrap());
    p.push("input.txt");
    std::io::BufReader::new(std::fs::File::open(p).unwrap())
}
