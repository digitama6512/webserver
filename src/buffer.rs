/// `Buffer` 环形缓冲区
///
/// 使用读指针(read_pos)和写指针(write_pos)管理数据：
/// - [0, read_pos): 已读取的预备空间
/// - [read_pos, write_pos): 待读取的有效数据
/// - [write_pos, buffer.len()): 可写入的空间
pub struct Buffer {
    buffer: Vec<u8>,
    read_pos: usize,
    write_pos: usize,
}

impl Buffer {
    /// `new` 创建指定初始大小的缓冲区
    /// - bufsize: 缓冲区大小
    pub fn new(bufsize: usize) -> Self {
        Buffer {
            // 缓冲区
            buffer: vec![0; bufsize],
            // 读指针
            read_pos: 0,
            // 写指针
            write_pos: 0,
        }
    }

    /// `readable_bytes` 获取环形缓冲区中可读区域的字节大小
    /// - return: 可读区域的字节大小
    pub fn readable_bytes(&self) -> usize {
        self.write_pos - self.read_pos
    }

    /// `writable_bytes` 获取环形缓冲区中可写区域的字节大小
    /// - return: 可写区域的字节大小
    pub fn writable_bytes(&self) -> usize {
        self.buffer.len() - self.write_pos
    }

    /// `prependable_bytes` 获取已读取的可重用空间
    /// - return: 可重用区域的字节大小
    pub fn prependable_bytes(&self) -> usize {
        self.read_pos
    }

    /// `peek` 获取当前可读数据的切片
    /// - return: 可读区域的切片
    pub fn peek(&self) -> &[u8] {
        &self.buffer[self.read_pos..self.write_pos]
    }

    

}
