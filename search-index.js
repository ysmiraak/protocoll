var searchIndex = {};
searchIndex["protocoll"] = {"doc":"some basic protocols and implementations for rust collections. inspired by\nclojure&#39;s design, albeit not persistent.","items":[[0,"map","protocoll","",null,null],[3,"VecSortedMap","protocoll::map","an array-map sorted by key.",null,null],[0,"set","protocoll","",null,null],[3,"VecSortedSet","protocoll::set","an array-set.",null,null],[8,"Map","protocoll","basic protocol for maps.",null,null],[10,"fun","","a map maps from keys to values.",0,null],[10,"inc","","adds `v` at `k`.",0,null],[10,"dec","","removes key `k`.",0,null],[11,"plus","","pours another collection into this one.",0,null],[10,"zero","","`clear`.",0,null],[10,"shrink","","`shrink_to_fit`.",0,null],[10,"update","","updates the value at `k` by `f`.",0,null],[11,"update_all","","updates all values by `f`",0,null],[11,"merge","","merges `coll` into this one, resolving conflicts by `f`.",0,null],[8,"MapMut","","",null,null],[10,"update_mut","","like [`Map::update`](trait.Map.html#tymethod.update) but can be more efficient.",1,null],[10,"update_all_mut","","like [`Map::update_all`](trait.Map.html#method.update_all) but can be more efficient.",1,null],[10,"merge_mut","","like [`Map::merge`](trait.Map.html#method.merge) but can be more efficient.",1,null],[8,"Set","","basic protocol for sets.",null,null],[10,"fun","","a set maps from items to themselves.",2,null],[10,"inc","","adds item `i`.",2,null],[10,"dec","","removes item `i`.",2,null],[11,"plus","","pours another collection into this one.",2,null],[10,"zero","","`clear`.",2,null],[10,"shrink","","`shrink_to_fit`.",2,null],[8,"Seq","","basic protocol for seqs.",null,null],[10,"fun","","a seq maps from indices to items. O(n) for `BinaryHeap`.",3,null],[10,"inc","","adds item `i`. both `Vec` and `VecDeque` grows to the right.",3,null],[10,"dec","","removes an item. for `Vec` it&#39;s the last one; for `VecDeque` the first;\nfor `BinaryHeap` it&#39;s the greatest one.",3,null],[11,"plus","","pours another collection into this one.",3,null],[10,"zero","","`clear`.",3,null],[10,"shrink","","`shrink_to_fit`.",3,null],[8,"Str","","basic protocol for strs;",null,null],[10,"inc","","appends char `c`.",4,null],[10,"dec","","pops the last char.",4,null],[10,"plus","","appends str `s`.",4,null],[10,"zero","","`clear`.",4,null],[10,"shrink","","`shrink_to_fit`.",4,null],[11,"plus","","pours another collection into this one.",0,null],[11,"update_all","","updates all values by `f`",0,null],[11,"merge","","merges `coll` into this one, resolving conflicts by `f`.",0,null],[11,"default","protocoll::map","",5,{"inputs":[],"output":{"name":"vecsortedmap"}}],[11,"clone","","",5,null],[11,"eq","","",5,null],[11,"ne","","",5,null],[11,"partial_cmp","","",5,null],[11,"lt","","",5,null],[11,"le","","",5,null],[11,"gt","","",5,null],[11,"ge","","",5,null],[11,"cmp","","",5,null],[11,"hash","","",5,null],[11,"new","","",5,{"inputs":[],"output":{"name":"self"}}],[11,"with_capacity","","",5,{"inputs":[{"name":"usize"}],"output":{"name":"self"}}],[11,"capacity","","",5,null],[11,"reserve","","",5,null],[11,"shrink_to_fit","","",5,null],[11,"clear","","",5,null],[11,"contains_key","","O(log(len))",5,null],[11,"get","","O(log(len))",5,null],[11,"get_mut","","O(log(len))",5,null],[11,"insert","","O(log(len)) when `k` already exists. O(len) for inserting a new entry,\ncaused by shifting all entries after it, which can be avoided by always\ninserting in order.",5,null],[11,"remove","","O(log(len)) when `k` does not exist. O(len) for removing an entry,\nbecause of the need for shifting all entries after it.",5,null],[11,"append","","",5,null],[11,"view_content","","a view for the underlying vec. `&amp;self` methods for `Vec` such as `get`\nand `split` can be accessed through this.",5,null],[11,"iter","","iterate over the underlying vec. note: iterator element type is **not**\n`(&amp;K,&amp;V)` but rather `&amp;(K,V)`. `iter_mut` is not supported for this\ncollection. see [`update_all_mut`](#method.update_all_mut) for\nthe same functionality.",5,null],[11,"len","","",5,null],[11,"is_empty","","",5,null],[11,"into_iter","","",5,null],[11,"extend","","",5,null],[11,"extend","","",5,null],[11,"from_iter","","",5,{"inputs":[{"name":"i"}],"output":{"name":"vecsortedmap"}}],[11,"index","","",5,null],[11,"fmt","","",5,null],[11,"fun","","",5,null],[11,"inc","","",5,null],[11,"dec","","",5,null],[11,"zero","","",5,null],[11,"shrink","","",5,null],[11,"update","","",5,null],[11,"update_all","","",5,null],[11,"update_mut","","",5,null],[11,"update_all_mut","","this makes up for the (intended) absence of `iter_mut`.",5,null],[11,"merge_mut","","",5,null],[11,"plus","protocoll","pours another collection into this one.",2,null],[11,"default","protocoll::set","",6,{"inputs":[],"output":{"name":"vecsortedset"}}],[11,"clone","","",6,null],[11,"eq","","",6,null],[11,"ne","","",6,null],[11,"partial_cmp","","",6,null],[11,"lt","","",6,null],[11,"le","","",6,null],[11,"gt","","",6,null],[11,"ge","","",6,null],[11,"cmp","","",6,null],[11,"hash","","",6,null],[11,"new","","",6,{"inputs":[],"output":{"name":"self"}}],[11,"with_capacity","","",6,{"inputs":[{"name":"usize"}],"output":{"name":"self"}}],[11,"capacity","","",6,null],[11,"reserve","","",6,null],[11,"shrink_to_fit","","",6,null],[11,"clear","","",6,null],[11,"contains","","O(log(len))",6,null],[11,"get","","O(log(len))",6,null],[11,"get_mut","","O(log(len))",6,null],[11,"insert","","O(log(len)) when `e` already exists. O(len) for inserting a new element,\ncaused by shifting all elements after it, which can be avoided by always\ninserting in order.",6,null],[11,"remove","","O(log(len)) when `e` does not exist. O(len) for removing an element,\nbecause of the need for shifting all elements after it.",6,null],[11,"append","","",6,null],[11,"view_content","","a view for the underlying vec. `&amp;self` methods for `Vec` such as `get`\nand `split` can be accessed through this.",6,null],[11,"iter","","iterate over the underlying vec.",6,null],[11,"len","","",6,null],[11,"is_empty","","",6,null],[11,"into_iter","","",6,null],[11,"extend","","",6,null],[11,"extend","","",6,null],[11,"from_iter","","",6,{"inputs":[{"name":"i"}],"output":{"name":"vecsortedset"}}],[11,"index","","",6,null],[11,"fmt","","",6,null],[11,"fun","","",6,null],[11,"inc","","",6,null],[11,"dec","","",6,null],[11,"zero","","",6,null],[11,"shrink","","",6,null],[11,"bitor","","union.",6,null],[11,"bitand","","intersection.",6,null],[11,"bitxor","","symmetric difference.",6,null],[11,"sub","","difference.",6,null],[11,"plus","protocoll","pours another collection into this one.",3,null]],"paths":[[8,"Map"],[8,"MapMut"],[8,"Set"],[8,"Seq"],[8,"Str"],[3,"VecSortedMap"],[3,"VecSortedSet"]]};
initSearch(searchIndex);
