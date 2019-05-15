# JALBT 

This is a try to create Bit Torrent client just to get skills on Rust language while learning.
I decided to do it because it seems to have all the topics that I would like to learn: 

- file management
- networking including http, tcp and udp
- client/server
- async communication
- multithreading

The idea is to do everything by myself and avoid the use of crates as far as possible.

## Base Documentation

- [BitTorrent specification](https://wiki.theory.org/index.php/BitTorrentSpecification)

- [BitTorrent BEP15](http://www.bittorrent.org/beps/bep_0015.html)

- [Blog article explaining how it works](https://allenkim67.github.io/programming/2016/05/04/how-to-make-your-own-bittorrent-client.html)


## Theory

### Torrent file

The torrent file is a bencoded file with the following structure

| Field | Type | Desc |
| ----- | ---- | ---- |
| announce | String | url with the announce of the tracker |
| info | info structure | structure with fields explained below |
| creation date | ? | (**optional**) the creation time of the torrent, in standard UNIX epoch format (integer, seconds since 1-Jan-1970 00:00:00 UTC) |
| comment | String | (**optional**) free-form textual comments of the author  |
| created by | String | (**optional**) name and version of the program used to create the .torrent |
| encoding | String | (**optional**) the string encoding format used to generate the pieces part of the info dictionary in the .torrent metafile |


**Info strcuture**

| Field | Type | Desc |
| ----- | ---- | ---- |
| piece length | u64  |  number of bytes in each piece |
| pieces | ByteBuf | string consisting of the concatenation of all 20-byte SHA1 hash values, one per piece (byte string, i.e. not urlencoded) |


**If is a sinlge file**

| Field | Type | Desc |
| ----- | ---- | ---- |
| name | String | file name |
| length | u64 | length of the file in bytes |
| md5sum | String | (**optional**) a 32-character hexadecimal string corresponding to the MD5 sum of the file. This is not used by BitTorrent at all, but it is included by some programs for greater compatibility. |


**If is a multiple file**

| Field | Type | Desc |
| ----- | ---- | ---- |
| name | String | the name of the directory in which to store all the files. This is purely advisory. |
| files | files struct | a list of dictionaries, one for each file. Each dictionary in this list contains the following keys:
|

**files struct**

| Field | Type | Desc |
| ----- | ---- | ---- |
| length | u64 | length of the file in bytes |
| md5sum | String | (**optional**) a 32-character hexadecimal string corresponding to the MD5 sum of the file. This is not used by BitTorrent at all, but it is included by some programs for greater compatibility. |
| path | ? | a list containing one or more string elements that together represent the path and filename. Each element in the list corresponds to either a directory name or (in the case of the final element) the filename. For example, a the file "dir1/dir2/file.ext" would consist of three string elements: "dir1", "dir2", and "file.ext". This is encoded as a bencoded list of strings such as l4:dir14:dir28:file.exte |





