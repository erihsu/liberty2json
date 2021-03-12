# liberty2json
Commandline tool that converts Synopsys Liberty format to JSON, which is inspired by [google skywater project](https://github.com/google/skywater-pdk).


## Why liberty2json
Liberty file is commonly used in most of state-of-art commercial P&R tools to store library and cell timing,power and area(PPA) information, usually in single file. So, you may not suprise to see the size of liberty file can reach to GBs in advanced technology. To avoide frequently spend long time reading this file, it's necessary to spilt them into multiple small .lib, ie, one common.lib to store basic technology information and many cell.lib to store separated cell information. This is also liberty2json considered.

Second, why use JSON instead of liberty? JSON is an ambiguous data format that widely used in many field and many programming language naturally support parsing this format. So I choose this file type as a alternative to .lib to make liberty file more easy to use.


## Liberty Syntax
The basic syntax of liberty format is consisted of **attribute** and **group**, where **attribute** syntax as

```
Attribute:
key: value
```
and **group** syntax as

```
Group:

named_group:

class(name){
	attribute1,
	attribute2,
	...
	group1-withoutname
	group2-withoutname
	...
}

or 

unnamed_group:

class(){
	attribute1,
	attribute2,
	...
	group1-hasname
	group2-hasname
}
```

In **attribute**, the value can be descripted in different ways, like simple string, float number, or quoted string.



## Usage

* Install liberty2json
```shell
cargo install liberty2json --git https://github.com/erihsu/liberty2json
```

* Convert liberty
```shell
liberty2json -o output_folder someliberty.lib 
```
if you not specify output_folder path, tool use the path of liberty file to generate json.


## Reference

[liberty syntax reference](https://people.eecs.berkeley.edu/~alanmi/publications/other/liberty07_03.pdf)
