(function() {var implementors = {};
implementors["eventmill"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"eventmill/aggregate/struct.Generation.html\" title=\"struct eventmill::aggregate::Generation\">Generation</a>","synthetic":true,"types":["eventmill::aggregate::Generation"]},{"text":"impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"eventmill/aggregate/struct.VersionedAggregate.html\" title=\"struct eventmill::aggregate::VersionedAggregate\">VersionedAggregate</a>&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a>,&nbsp;</span>","synthetic":true,"types":["eventmill::aggregate::VersionedAggregate"]},{"text":"impl&lt;C, A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"eventmill/command/struct.DomainCommand.html\" title=\"struct eventmill::command::DomainCommand\">DomainCommand</a>&lt;C, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;A as <a class=\"trait\" href=\"eventmill/aggregate/trait.WithAggregateId.html\" title=\"trait eventmill::aggregate::WithAggregateId\">WithAggregateId</a>&gt;::<a class=\"type\" href=\"eventmill/aggregate/trait.WithAggregateId.html#associatedtype.Id\" title=\"type eventmill::aggregate::WithAggregateId::Id\">Id</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a>,&nbsp;</span>","synthetic":true,"types":["eventmill::command::DomainCommand"]},{"text":"impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"eventmill/dispatch/struct.Core.html\" title=\"struct eventmill::dispatch::Core\">Core</a>&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a>,&nbsp;</span>","synthetic":true,"types":["eventmill::dispatch::Core"]},{"text":"impl&lt;R, W, H&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"enum\" href=\"eventmill/dispatch/enum.CoreError.html\" title=\"enum eventmill::dispatch::CoreError\">CoreError</a>&lt;R, W, H&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;H: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;W: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a>,&nbsp;</span>","synthetic":true,"types":["eventmill::dispatch::CoreError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"eventmill/event/struct.Sequence.html\" title=\"struct eventmill::event::Sequence\">Sequence</a>","synthetic":true,"types":["eventmill::event::Sequence"]},{"text":"impl&lt;E, A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"eventmill/event/struct.DomainEvent.html\" title=\"struct eventmill::event::DomainEvent\">DomainEvent</a>&lt;E, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;A as <a class=\"trait\" href=\"eventmill/aggregate/trait.WithAggregateId.html\" title=\"trait eventmill::aggregate::WithAggregateId\">WithAggregateId</a>&gt;::<a class=\"type\" href=\"eventmill/aggregate/trait.WithAggregateId.html#associatedtype.Id\" title=\"type eventmill::aggregate::WithAggregateId::Id\">Id</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a>,&nbsp;</span>","synthetic":true,"types":["eventmill::event::DomainEvent"]},{"text":"impl&lt;E, A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"eventmill/event/struct.NewEvent.html\" title=\"struct eventmill::event::NewEvent\">NewEvent</a>&lt;E, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;A as <a class=\"trait\" href=\"eventmill/aggregate/trait.WithAggregateId.html\" title=\"trait eventmill::aggregate::WithAggregateId\">WithAggregateId</a>&gt;::<a class=\"type\" href=\"eventmill/aggregate/trait.WithAggregateId.html#associatedtype.Id\" title=\"type eventmill::aggregate::WithAggregateId::Id\">Id</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a>,&nbsp;</span>","synthetic":true,"types":["eventmill::event::NewEvent"]},{"text":"impl&lt;E, A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"eventmill/inmemory_store/struct.InMemoryStore.html\" title=\"struct eventmill::inmemory_store::InMemoryStore\">InMemoryStore</a>&lt;E, A&gt;","synthetic":true,"types":["eventmill::inmemory_store::InMemoryStore"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"enum\" href=\"eventmill/inmemory_store/enum.Error.html\" title=\"enum eventmill::inmemory_store::Error\">Error</a>","synthetic":true,"types":["eventmill::inmemory_store::Error"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"struct\" href=\"eventmill/metadata/struct.Attribute.html\" title=\"struct eventmill::metadata::Attribute\">Attribute</a>","synthetic":true,"types":["eventmill::metadata::Attribute"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/panic/trait.RefUnwindSafe.html\" title=\"trait std::panic::RefUnwindSafe\">RefUnwindSafe</a> for <a class=\"enum\" href=\"eventmill/metadata/enum.Value.html\" title=\"enum eventmill::metadata::Value\">Value</a>","synthetic":true,"types":["eventmill::metadata::Value"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()