use ssh::*;

const SSH_FXP_INIT:usize           = 1;
const SSH_FXP_VERSION:usize        = 2;
const SSH_FXP_OPEN:usize           = 3;
const SSH_FXP_CLOSE:usize          = 4;
const SSH_FXP_READ:usize           = 5;
const SSH_FXP_WRITE:usize          = 6;
const SSH_FXP_LSTAT:usize          = 7;
const SSH_FXP_FSTAT:usize          = 8;
const SSH_FXP_SETSTAT:usize        = 9;
const SSH_FXP_FSETSTAT:usize       = 10;
const SSH_FXP_OPENDIR:usize        = 11;
const SSH_FXP_READDIR:usize        = 12;
const SSH_FXP_REMOVE:usize         = 13;
const SSH_FXP_MKDIR:usize          = 14;
const SSH_FXP_RMDIR:usize          = 15;
const SSH_FXP_REALPATH:usize       = 16;
const SSH_FXP_STAT:usize           = 17;
const SSH_FXP_RENAME:usize         = 18;
const SSH_FXP_READLINK:usize       = 19;
const SSH_FXP_SYMLINK:usize        = 20;
const SSH_FXP_STATUS:usize         = 101;
const SSH_FXP_HANDLE:usize         = 102;
const SSH_FXP_DATA:usize           = 103;
const SSH_FXP_NAME:usize           = 104;
const SSH_FXP_ATTRS:usize          = 105;
const SSH_FXP_EXTENDED:usize       = 200;
const SSH_FXP_EXTENDED_REPLY:usize = 201;

struct packet {
    length: u32,
    typ:    u8,
    data:   &[u8],
}
