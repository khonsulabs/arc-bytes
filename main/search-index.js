var searchIndex = JSON.parse('{\
"arc_bytes":{"doc":"arc-bytes","t":[12,3,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,5,11,11,0,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["0","ArcBytes","Iter","OwnedBytes","as_slice","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrowed","clone","clone","clone_into","clone_into","cmp","cmp","default","deref","deref","deref_mut","deserialize","deserialize","eq","eq","eq","eq","eq","eq","eq","eq","eq","fmt","fmt","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","from","hash","hash","into","into","into","into_iter","into_iter","into_owned","into_vec","iter","ne","new","next","owned","partial_cmp","partial_cmp","partial_cmp","partial_cmp","partial_cmp","partial_cmp","partial_cmp","partial_cmp","print_bytes","read","read_bytes","serde","serialize","serialize","slice","split_at","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","0","0","Bytes","CowBytes","borrow","borrow","borrow_mut","borrow_mut","clone","clone","clone_into","clone_into","cmp","cmp","default","default","deref","deref","deref_mut","deref_mut","deserialize","deserialize","eq","eq","fmt","fmt","from","from","from","from","from","from","from","from","from","hash","hash","into","into","into_cow","into_vec","into_vec","ne","ne","partial_cmp","partial_cmp","serialize","serialize","to_owned","to_owned","try_from","try_from","try_into","try_into","type_id","type_id"],"q":["arc_bytes","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","arc_bytes::serde","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","An immutable buffer of bytes that can be cloned, sliced, …","An iterator for an <code>ArcBytes</code>.","An instance of <code>ArcBytes</code> that is not borrowing its …","Returns this instance as a slice of <code>u8</code>s.","","","","","","","","","Returns a borrowed instance.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Converts this instance into a static lifetime, …","Converts this instance into a <code>Vec&lt;u8&gt;</code>, attempting to do so …","Returns an iterator for the contained bytes.","","Returns an empty instance.","","Returns an instance with the owned bytes.","","","","","","","","","Formats the bytes contained in <code>slice</code> into the provided …","","Reads <code>count</code> bytes from the front of the bytes, returning a …","Efficient serialization implementation, ensuring bytes are …","","","Returns a slice of these bytes as its own <code>ArcBytes</code> …","Splits the bytes into two parts at <code>offset</code>. This method …","","Converts a clone of this instance into a static lifetime.","","","","","","","","","","","","","A <code>Vec&lt;u8&gt;</code> wrapper that supports serializing efficiently in …","A <code>Cow&lt;&#39;a, [u8]&gt;</code> wrapper that supports serializing …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the underlying Cow.","Returns the underlying Vec.","Returns the underlying Vec inside of the Cow, or clones …","","","","","","","","","","","","","",""],"i":[1,0,0,0,2,3,2,2,1,1,3,2,1,2,2,1,2,1,2,1,2,2,1,1,2,1,2,2,2,2,2,1,1,1,1,2,1,3,2,2,2,2,2,2,2,2,2,2,1,1,1,1,1,2,1,3,2,1,3,2,2,2,2,1,2,3,2,2,2,2,2,1,1,1,1,0,2,2,0,2,1,2,2,2,2,1,3,2,1,3,2,1,3,2,1,4,5,0,0,4,5,4,5,4,5,4,5,4,5,4,5,4,5,4,5,4,5,4,5,4,5,4,4,4,4,4,5,5,5,5,4,5,4,5,5,4,5,4,5,4,5,4,5,4,5,4,5,4,5,4,5],"f":[null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["arcbytes",3]],[[],["ownedbytes",3]],[[]],[[]],[[],["ordering",4]],[[["ownedbytes",3]],["ordering",4]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["bool",15]],[[],["bool",15]],[[["arcbytes",3]],["bool",15]],[[],["bool",15]],[[],["bool",15]],[[],["bool",15]],[[],["bool",15]],[[],["bool",15]],[[["ownedbytes",3]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[["cow",4]]],[[]],[[["vec",3,[["u8",15]]]]],[[["cowbytes",3]]],[[]],[[["str",15]]],[[["bytes",3]]],[[]],[[]],[[["string",3]]],[[["vec",3,[["u8",15]]]]],[[["arcbytes",3]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["arcbytes",3]],[[],["vec",3,[["u8",15]]]],[[],["iter",3]],[[["ownedbytes",3]],["bool",15]],[[]],[[],["option",4]],[[["vec",3,[["u8",15]]]]],[[["arcbytes",3]],["option",4,[["ordering",4]]]],[[],["option",4,[["ordering",4]]]],[[],["option",4,[["ordering",4]]]],[[],["option",4,[["ordering",4]]]],[[],["option",4,[["ordering",4]]]],[[["ownedbytes",3]],["option",4,[["ordering",4]]]],[[],["option",4,[["ordering",4]]]],[[],["option",4,[["ordering",4]]]],[[],["result",6]],[[],["result",6,[["usize",15]]]],[[["usize",15]],["result",4,[["error",3]]]],null,[[],["result",4]],[[],["result",4]],[[["rangebounds",8,[["usize",15]]]]],[[["usize",15]]],[[]],[[],["arcbytes",3]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null,[[]],[[]],[[]],[[]],[[],["bytes",3]],[[],["cowbytes",3]],[[]],[[]],[[["bytes",3]],["ordering",4]],[[["cowbytes",3]],["ordering",4]],[[],["bytes",3]],[[],["cowbytes",3]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[["bytes",3]],["bool",15]],[[["cowbytes",3]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["arcbytes",3]]],[[["vec",3,[["u8",15]]]]],[[["cowbytes",3]]],[[]],[[]],[[["bytes",3]]],[[]],[[["vec",3,[["u8",15]]]]],[[]],[[]],[[]],[[]],[[]],[[],["cow",4]],[[],["vec",3,[["u8",15]]]],[[],["vec",3,[["u8",15]]]],[[["bytes",3]],["bool",15]],[[["cowbytes",3]],["bool",15]],[[["bytes",3]],["option",4,[["ordering",4]]]],[[["cowbytes",3]],["option",4,[["ordering",4]]]],[[],["result",4]],[[],["result",4]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]]],"p":[[3,"OwnedBytes"],[3,"ArcBytes"],[3,"Iter"],[3,"Bytes"],[3,"CowBytes"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};