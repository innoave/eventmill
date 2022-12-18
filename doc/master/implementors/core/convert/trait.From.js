(function() {var implementors = {
"eventmill":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.66.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"eventmill/struct.Sequence.html\" title=\"struct eventmill::Sequence\">Sequence</a>&gt; for <a class=\"struct\" href=\"eventmill/struct.Generation.html\" title=\"struct eventmill::Generation\">Generation</a>"],["impl&lt;C, A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.66.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;(C, &amp;<a class=\"struct\" href=\"eventmill/struct.VersionedAggregate.html\" title=\"struct eventmill::VersionedAggregate\">VersionedAggregate</a>&lt;A&gt;)&gt; for <a class=\"struct\" href=\"eventmill/struct.DomainCommand.html\" title=\"struct eventmill::DomainCommand\">DomainCommand</a>&lt;C, A&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"eventmill/trait.WithAggregateId.html\" title=\"trait eventmill::WithAggregateId\">WithAggregateId</a>,</span>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.66.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"eventmill/struct.Generation.html\" title=\"struct eventmill::Generation\">Generation</a>&gt; for <a class=\"struct\" href=\"eventmill/struct.Sequence.html\" title=\"struct eventmill::Sequence\">Sequence</a>"],["impl&lt;E, A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.66.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;(&lt;A as <a class=\"trait\" href=\"eventmill/trait.WithAggregateId.html\" title=\"trait eventmill::WithAggregateId\">WithAggregateId</a>&gt;::<a class=\"associatedtype\" href=\"eventmill/trait.WithAggregateId.html#associatedtype.Id\" title=\"type eventmill::WithAggregateId::Id\">Id</a>, E)&gt; for <a class=\"struct\" href=\"eventmill/struct.NewEvent.html\" title=\"struct eventmill::NewEvent\">NewEvent</a>&lt;E, A&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"eventmill/trait.WithAggregateId.html\" title=\"trait eventmill::WithAggregateId\">WithAggregateId</a>,</span>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.66.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.66.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"enum\" href=\"eventmill/metadata/enum.Value.html\" title=\"enum eventmill::metadata::Value\">Value</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.66.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.66.0/std/primitive.i32.html\">i32</a>&gt; for <a class=\"enum\" href=\"eventmill/metadata/enum.Value.html\" title=\"enum eventmill::metadata::Value\">Value</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.66.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.66.0/std/primitive.i64.html\">i64</a>&gt; for <a class=\"enum\" href=\"eventmill/metadata/enum.Value.html\" title=\"enum eventmill::metadata::Value\">Value</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.66.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.66.0/std/primitive.f32.html\">f32</a>&gt; for <a class=\"enum\" href=\"eventmill/metadata/enum.Value.html\" title=\"enum eventmill::metadata::Value\">Value</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.66.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.66.0/std/primitive.f64.html\">f64</a>&gt; for <a class=\"enum\" href=\"eventmill/metadata/enum.Value.html\" title=\"enum eventmill::metadata::Value\">Value</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.66.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.66.0/std/primitive.bool.html\">bool</a>&gt; for <a class=\"enum\" href=\"eventmill/metadata/enum.Value.html\" title=\"enum eventmill::metadata::Value\">Value</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.66.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://docs.rs/chrono/latest/chrono/naive/date/struct.NaiveDate.html\" title=\"struct chrono::naive::date::NaiveDate\">NaiveDate</a>&gt; for <a class=\"enum\" href=\"eventmill/metadata/enum.Value.html\" title=\"enum eventmill::metadata::Value\">Value</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.66.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://docs.rs/chrono/latest/chrono/datetime/struct.DateTime.html\" title=\"struct chrono::datetime::DateTime\">DateTime</a>&lt;<a class=\"struct\" href=\"https://docs.rs/chrono/latest/chrono/offset/utc/struct.Utc.html\" title=\"struct chrono::offset::utc::Utc\">Utc</a>&gt;&gt; for <a class=\"enum\" href=\"eventmill/metadata/enum.Value.html\" title=\"enum eventmill::metadata::Value\">Value</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.66.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.66.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"enum\" href=\"eventmill/metadata/enum.Value.html\" title=\"enum eventmill::metadata::Value\">Value</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.66.0/alloc/alloc/struct.Global.html\" title=\"struct alloc::alloc::Global\">Global</a>&gt;&gt; for <a class=\"enum\" href=\"eventmill/metadata/enum.Value.html\" title=\"enum eventmill::metadata::Value\">Value</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.66.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.66.0/std/collections/hash/map/struct.HashMap.html\" title=\"struct std::collections::hash::map::HashMap\">HashMap</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.66.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"enum\" href=\"eventmill/metadata/enum.Value.html\" title=\"enum eventmill::metadata::Value\">Value</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.66.0/std/collections/hash/map/struct.RandomState.html\" title=\"struct std::collections::hash::map::RandomState\">RandomState</a>&gt;&gt; for <a class=\"enum\" href=\"eventmill/metadata/enum.Value.html\" title=\"enum eventmill::metadata::Value\">Value</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.66.0/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;(<a class=\"struct\" href=\"https://doc.rust-lang.org/1.66.0/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"enum\" href=\"eventmill/metadata/enum.Value.html\" title=\"enum eventmill::metadata::Value\">Value</a>)&gt; for <a class=\"struct\" href=\"eventmill/metadata/struct.Attribute.html\" title=\"struct eventmill::metadata::Attribute\">Attribute</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()