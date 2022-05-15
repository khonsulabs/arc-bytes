(function() {var implementors = {};
implementors["arc_bytes"] = [{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"arc_bytes/struct.ArcBytes.html\" title=\"struct arc_bytes::ArcBytes\">ArcBytes</a>&lt;'a&gt;","synthetic":false,"types":["arc_bytes::ArcBytes"]},{"text":"impl&lt;'a, 'b, const SIZE:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>&lt;&amp;'b <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.array.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.array.html\">; SIZE]</a>&gt; for <a class=\"struct\" href=\"arc_bytes/struct.ArcBytes.html\" title=\"struct arc_bytes::ArcBytes\">ArcBytes</a>&lt;'a&gt;","synthetic":false,"types":["arc_bytes::ArcBytes"]},{"text":"impl&lt;'b, 'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.slice.html\">&amp;'b [</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"arc_bytes/struct.ArcBytes.html\" title=\"struct arc_bytes::ArcBytes\">ArcBytes</a>&lt;'a&gt;","synthetic":false,"types":["arc_bytes::ArcBytes"]},{"text":"impl&lt;'a, 'b&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>&lt;<a class=\"struct\" href=\"arc_bytes/struct.ArcBytes.html\" title=\"struct arc_bytes::ArcBytes\">ArcBytes</a>&lt;'b&gt;&gt; for <a class=\"struct\" href=\"arc_bytes/struct.ArcBytes.html\" title=\"struct arc_bytes::ArcBytes\">ArcBytes</a>&lt;'a&gt;","synthetic":false,"types":["arc_bytes::ArcBytes"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>&lt;<a class=\"struct\" href=\"arc_bytes/struct.OwnedBytes.html\" title=\"struct arc_bytes::OwnedBytes\">OwnedBytes</a>&gt; for <a class=\"struct\" href=\"arc_bytes/struct.OwnedBytes.html\" title=\"struct arc_bytes::OwnedBytes\">OwnedBytes</a>","synthetic":false,"types":["arc_bytes::OwnedBytes"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"arc_bytes/struct.OwnedBytes.html\" title=\"struct arc_bytes::OwnedBytes\">OwnedBytes</a>","synthetic":false,"types":["arc_bytes::OwnedBytes"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.slice.html\">&amp;'a [</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"arc_bytes/struct.OwnedBytes.html\" title=\"struct arc_bytes::OwnedBytes\">OwnedBytes</a>","synthetic":false,"types":["arc_bytes::OwnedBytes"]},{"text":"impl&lt;'a, const SIZE:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>&lt;&amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.array.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.array.html\">; SIZE]</a>&gt; for <a class=\"struct\" href=\"arc_bytes/struct.OwnedBytes.html\" title=\"struct arc_bytes::OwnedBytes\">OwnedBytes</a>","synthetic":false,"types":["arc_bytes::OwnedBytes"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>&lt;<a class=\"struct\" href=\"arc_bytes/serde/struct.Bytes.html\" title=\"struct arc_bytes::serde::Bytes\">Bytes</a>&gt; for <a class=\"struct\" href=\"arc_bytes/serde/struct.Bytes.html\" title=\"struct arc_bytes::serde::Bytes\">Bytes</a>","synthetic":false,"types":["arc_bytes::serde::Bytes"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"arc_bytes/serde/struct.Bytes.html\" title=\"struct arc_bytes::serde::Bytes\">Bytes</a>","synthetic":false,"types":["arc_bytes::serde::Bytes"]},{"text":"impl&lt;'a, 'b, const SIZE:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>&lt;&amp;'b <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.array.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.array.html\">; SIZE]</a>&gt; for <a class=\"struct\" href=\"arc_bytes/serde/struct.Bytes.html\" title=\"struct arc_bytes::serde::Bytes\">Bytes</a>","synthetic":false,"types":["arc_bytes::serde::Bytes"]},{"text":"impl&lt;'b, 'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.slice.html\">&amp;'b [</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"arc_bytes/serde/struct.Bytes.html\" title=\"struct arc_bytes::serde::Bytes\">Bytes</a>","synthetic":false,"types":["arc_bytes::serde::Bytes"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>&lt;<a class=\"struct\" href=\"arc_bytes/serde/struct.CowBytes.html\" title=\"struct arc_bytes::serde::CowBytes\">CowBytes</a>&lt;'a&gt;&gt; for <a class=\"struct\" href=\"arc_bytes/serde/struct.CowBytes.html\" title=\"struct arc_bytes::serde::CowBytes\">CowBytes</a>&lt;'a&gt;","synthetic":false,"types":["arc_bytes::serde::CowBytes"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.slice.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"arc_bytes/serde/struct.CowBytes.html\" title=\"struct arc_bytes::serde::CowBytes\">CowBytes</a>&lt;'a&gt;","synthetic":false,"types":["arc_bytes::serde::CowBytes"]},{"text":"impl&lt;'a, 'b, const SIZE:&nbsp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>&lt;&amp;'b <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.array.html\">[</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.array.html\">; SIZE]</a>&gt; for <a class=\"struct\" href=\"arc_bytes/serde/struct.CowBytes.html\" title=\"struct arc_bytes::serde::CowBytes\">CowBytes</a>&lt;'a&gt;","synthetic":false,"types":["arc_bytes::serde::CowBytes"]},{"text":"impl&lt;'b, 'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.60.0/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.slice.html\">&amp;'b [</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.u8.html\">u8</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/1.60.0/std/primitive.slice.html\">]</a>&gt; for <a class=\"struct\" href=\"arc_bytes/serde/struct.CowBytes.html\" title=\"struct arc_bytes::serde::CowBytes\">CowBytes</a>&lt;'a&gt;","synthetic":false,"types":["arc_bytes::serde::CowBytes"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()