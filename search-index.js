var searchIndex = {};
searchIndex["protocoll"] = {"doc":"some basic protocols and implementations for rust collections. inspired by\nclojure&#39;s design, albeit not persistent.","items":[[8,"Map","protocoll","basic protocol for maps.",null,null],[10,"fun","","a map maps from keys to values.",0,null],[10,"inc","","adds entry `kv`.",0,null],[10,"dec","","removes key `k`.",0,null],[11,"plus","","pours another collection into this one.",0,null],[10,"zero","","`clear`.",0,null],[10,"shrink","","`shrink_to_fit`.",0,null],[10,"update","","like clojure&#39;s [update](http://clojuredocs.org/clojure.core/update).\n# example\n```\nuse protocoll::Map;\nuse std::collections::HashMap;\nlet m = [0,0,0,1,1,0,0,0].iter()\n   .fold(HashMap::new(), |m,&amp;k| Map::update\n         (m,k, |opt_n| 1 + opt_n.unwrap_or(0)));\nassert_eq!(6, m[&amp;0]);\nassert_eq!(2, m[&amp;1]);\n```",0,null],[11,"merge","","like clojure&#39;s [merge-with](http://clojuredocs.org/clojure.core/merge-with).\n# example\n```\nuse protocoll::Map;\nuse std::collections::HashMap;\nuse std::ops::Add;\nlet m = [0,0,0,1,1,0,0,0].iter()\n   .fold(HashMap::new(), |m,&amp;k| Map::update\n         (m,k, |opt_n| 1 + opt_n.unwrap_or(0)));\nlet m = Map::merge(m.clone(), m, usize::add);\nassert_eq!(12, m[&amp;0]);\nassert_eq!(4, m[&amp;1]);\n```",0,null],[8,"Set","","basic protocol for sets.",null,null],[10,"fun","","a set maps from items to themselves.",1,null],[10,"inc","","adds item `i`.",1,null],[10,"dec","","removes item `i`.",1,null],[11,"plus","","pours another collection into this one.",1,null],[10,"zero","","`clear`.",1,null],[10,"shrink","","`shrink_to_fit`.",1,null],[8,"Seq","","basic protocol for seqs.",null,null],[10,"fun","","a seq maps from indices to items. O(n) for `BinaryHeap`.",2,null],[10,"inc","","adds item `i`. both `Vec` and `VecDeque` grows to the right.",2,null],[10,"dec","","removes an item. for `Vec` it&#39;s the last one; for `VecDeque` the first;\nfor `BinaryHeap` it&#39;s the greatest one.",2,null],[11,"plus","","pours another collection into this one.",2,null],[10,"zero","","`clear`.",2,null],[10,"shrink","","`shrink_to_fit`.",2,null],[8,"Str","","basic protocol for strs;",null,null],[10,"inc","","appends char `c`.",3,null],[10,"dec","","pops the last char.",3,null],[10,"plus","","appends str `s`.",3,null],[10,"zero","","`clear`.",3,null],[10,"shrink","","`shrink_to_fit`.",3,null],[11,"plus","","pours another collection into this one.",0,null],[11,"merge","","like clojure&#39;s [merge-with](http://clojuredocs.org/clojure.core/merge-with).\n# example\n```\nuse protocoll::Map;\nuse std::collections::HashMap;\nuse std::ops::Add;\nlet m = [0,0,0,1,1,0,0,0].iter()\n   .fold(HashMap::new(), |m,&amp;k| Map::update\n         (m,k, |opt_n| 1 + opt_n.unwrap_or(0)));\nlet m = Map::merge(m.clone(), m, usize::add);\nassert_eq!(12, m[&amp;0]);\nassert_eq!(4, m[&amp;1]);\n```",0,null],[11,"plus","","pours another collection into this one.",1,null],[11,"plus","","pours another collection into this one.",2,null]],"paths":[[8,"Map"],[8,"Set"],[8,"Seq"],[8,"Str"]]};
initSearch(searchIndex);