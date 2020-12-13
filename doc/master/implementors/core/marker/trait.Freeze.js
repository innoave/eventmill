(function() {var implementors = {};
implementors["eventmill"] = [{"text":"impl Freeze for Generation","synthetic":true,"types":[]},{"text":"impl&lt;S&gt; Freeze for VersionedAggregate&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Freeze,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;C, A&gt; Freeze for DomainCommand&lt;C, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;C: Freeze,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;A as WithAggregateId&gt;::Id: Freeze,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;E, A&gt; Freeze for DomainEvent&lt;E, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: Freeze,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;A as WithAggregateId&gt;::Id: Freeze,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, E, A&gt; Freeze for DomainEventView&lt;'a, E, A&gt;","synthetic":true,"types":[]},{"text":"impl&lt;E, A&gt; Freeze for NewEvent&lt;E, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: Freeze,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;A as WithAggregateId&gt;::Id: Freeze,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Freeze for Sequence","synthetic":true,"types":[]},{"text":"impl&lt;S&gt; Freeze for Core&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Freeze,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;R, W, H&gt; Freeze for CoreError&lt;R, W, H&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;H: Freeze,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Freeze,<br>&nbsp;&nbsp;&nbsp;&nbsp;W: Freeze,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;E, A&gt; Freeze for InMemoryStore&lt;E, A&gt;","synthetic":true,"types":[]},{"text":"impl Freeze for Error","synthetic":true,"types":[]},{"text":"impl Freeze for Attribute","synthetic":true,"types":[]},{"text":"impl Freeze for Value","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()