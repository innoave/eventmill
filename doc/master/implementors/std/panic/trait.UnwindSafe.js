(function() {var implementors = {};
implementors["eventmill"] = [{"text":"impl UnwindSafe for Generation","synthetic":true,"types":[]},{"text":"impl&lt;S&gt; UnwindSafe for VersionedAggregate&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;C, A&gt; UnwindSafe for DomainCommand&lt;C, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: UnwindSafe,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;A as WithAggregateId&gt;::Id: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;E, A&gt; UnwindSafe for DomainEvent&lt;E, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: UnwindSafe,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;A as WithAggregateId&gt;::Id: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, E, A&gt; UnwindSafe for DomainEventView&lt;'a, E, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: RefUnwindSafe,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;A as WithAggregateId&gt;::Id: RefUnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;E, A&gt; UnwindSafe for NewEvent&lt;E, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: UnwindSafe,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;A as WithAggregateId&gt;::Id: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Sequence","synthetic":true,"types":[]},{"text":"impl&lt;S&gt; UnwindSafe for Core&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;R, W, H&gt; UnwindSafe for CoreError&lt;R, W, H&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;H: UnwindSafe,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: UnwindSafe,<br>&nbsp;&nbsp;&nbsp;&nbsp;W: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;E, A&gt; UnwindSafe for InMemoryStore&lt;E, A&gt;","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Error","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Attribute","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Value","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()