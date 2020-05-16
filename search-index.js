var searchIndex={};
searchIndex["eventmill"] = {"doc":"Abstractions","i":[[24,"EventType","eventmill","",null,null],[0,"aggregate","","",null,null],[3,"Generation","eventmill::aggregate","",null,null],[3,"VersionedAggregate","","",null,null],[6,"AggregateIdOf","","",null,null],[8,"AggregateType","","",null,null],[11,"aggregate_type","","",0,[[],["str"]]],[8,"WithAggregateId","","",null,null],[16,"Id","","",1,null],[10,"aggregate_id","","",1,[[["self"]]]],[8,"AggregateState","","",null,null],[10,"generation","","",2,[[["self"]],["generation"]]],[10,"generation_mut","","",2,[[["self"]],["generation"]]],[8,"Aggregate","","",null,null],[10,"apply_event","","",3,[[["self"],["domainevent"]]]],[11,"apply_all_events","","",3,[[["self"]]]],[8,"InitializeAggregate","","",null,null],[16,"State","","",4,null],[10,"initialize","","",4,[[["aggregateidof"]]]],[11,"number","","",5,[[],["u64"]]],[11,"increment","","",5,[[["self"]]]],[11,"restore","","",6,[[["s"],["u64"]],["self"]]],[11,"unwrap","","",6,[[],["s"]]],[11,"generation","","",6,[[["self"]],["generation"]]],[11,"state","","",6,[[["self"]],["s"]]],[0,"command","eventmill","",null,null],[3,"DomainCommand","eventmill::command","",null,null],[12,"aggregate_id","","",7,null],[12,"aggregate_generation","","",7,null],[12,"data","","",7,null],[6,"EventOf","","",null,null],[8,"HandleCommand","","",null,null],[16,"Event","","",8,null],[16,"Error","","",8,null],[16,"Context","","",8,null],[10,"handle_command","","",8,[[["c"],["self"]],[["vec",["newevent"]],["result",["vec"]]]]],[11,"new","","",7,[[["c"],["aggregateidof"],["generation"]],["self"]]],[0,"dispatch","eventmill","",null,null],[3,"Core","eventmill::dispatch","",null,null],[4,"CoreError","","",null,null],[13,"ReplayAggregateFailed","","",9,null],[13,"AppendEventsFailed","","",9,null],[13,"HandleCommandFailed","","",9,null],[13,"GenerationConflict","","",9,null],[12,"assumed","eventmill::dispatch::CoreError","",10,null],[12,"actual","","",10,null],[6,"CoreDispatchError","eventmill::dispatch","",null,null],[8,"DispatchEvent","","",null,null],[10,"dispatch","","",11,[[["self"],["domainevent"]]]],[8,"DispatchCommand","","",null,null],[16,"Context","","",12,null],[16,"Output","","",12,null],[16,"Error","","",12,null],[10,"dispatch_command","","",12,[[["c"],["self"]],["result"]]],[11,"new","","",13,[[["s"]],["self"]]],[0,"event","eventmill","",null,null],[3,"Sequence","eventmill::event","",null,null],[3,"DomainEvent","","",null,null],[12,"aggregate_id","","",14,null],[12,"sequence","","",14,null],[12,"time","","",14,null],[12,"data","","",14,null],[12,"metadata","","",14,null],[3,"NewEvent","","",null,null],[12,"aggregate_id","","",15,null],[12,"data","","",15,null],[5,"wrap_events","","",null,[[["sequence"]]]],[5,"wrap_events_with_metadata","","",null,[[["sequence"],["metadata"]]]],[8,"EventType","","",null,null],[10,"event_type_version","","",16,[[["self"]],["str"]]],[10,"event_type","","",16,[[["self"]],["str"]]],[10,"event_source","","",16,[[["self"]],["str"]]],[11,"number","","",17,[[],["u64"]]],[11,"next_value","","",17,[[["self"]],["self"]]],[11,"new","","",14,[[["datetime",["utc"]],["utc"],["sequence"],["aggregateidof"],["e"]],["self"]]],[11,"new_now","","",14,[[["sequence"],["aggregateidof"],["e"]],["self"]]],[11,"with_metadata","","",14,[[["m"]],["self"]]],[11,"unwrap","","",14,[[]]],[11,"aggregate_type","","",14,[[["self"]],["str"]]],[11,"aggregate_id","","",14,[[["self"]],["aggregateidof"]]],[11,"sequence","","",14,[[["self"]],["sequence"]]],[11,"time","","",14,[[["self"]],[["utc"],["datetime",["utc"]]]]],[11,"data","","",14,[[["self"]],["e"]]],[11,"metadata","","",14,[[["self"]],["metadata"]]],[11,"transmute","","",14,[[["u"],["self"]],["domainevent"]]],[11,"new","","",15,[[["aggregateidof"],["e"]],["self"]]],[11,"unwrap","","",15,[[]]],[11,"aggregate_id","","",15,[[["self"]],["aggregateidof"]]],[11,"data","","",15,[[["self"]],["e"]]],[0,"inmemory_store","eventmill","",null,null],[3,"InMemoryStore","eventmill::inmemory_store","",null,null],[4,"Error","","",null,null],[13,"NoReadAccess","","",18,null],[13,"NoWriteAccess","","",18,null],[6,"InMemoryStoreError","","",null,null],[11,"new","","",19,[[],["self"]]],[11,"with_events","","",19,[[],["self"]]],[0,"metadata","eventmill","",null,null],[3,"Attribute","eventmill::metadata","",null,null],[12,"key","","",20,null],[12,"value","","",20,null],[4,"Value","","",null,null],[13,"String","","",21,null],[13,"Integer","","",21,null],[13,"Long","","",21,null],[13,"Float","","",21,null],[13,"Double","","",21,null],[13,"Boolean","","",21,null],[13,"Date","","",21,null],[13,"DateTime","","",21,null],[13,"List","","",21,null],[13,"Map","","",21,null],[6,"Metadata","","",null,null],[6,"Key","","",null,null],[8,"Attributes","","",null,null],[11,"into_tuple","","",20,[[]]],[0,"query","eventmill","",null,null],[8,"ReceiveEvent","eventmill::query","",null,null],[10,"receive_event","","",22,[[["domainevent"],["self"]]]],[0,"store","eventmill","",null,null],[6,"EventSinkError","eventmill::store","",null,null],[6,"EventSourceError","","",null,null],[8,"EventSink","","",null,null],[16,"Error","","",23,null],[10,"append","","",23,[[["self"],["domainevent"]],["result"]]],[10,"append_batch","","",23,[[["self"]],["result"]]],[8,"EventSource","","",null,null],[16,"Error","","",24,null],[10,"read_events","","",24,[[["r"],["self"],["aggregateidof"]],["result"]]],[0,"test_support","eventmill","",null,null],[11,"from","eventmill::aggregate","",5,[[["t"]],["t"]]],[11,"into","","",5,[[],["u"]]],[11,"to_owned","","",5,[[["self"]],["t"]]],[11,"clone_into","","",5,[[["self"],["t"]]]],[11,"to_string","","",5,[[["self"]],["string"]]],[11,"try_from","","",5,[[["u"]],["result"]]],[11,"try_into","","",5,[[],["result"]]],[11,"borrow","","",5,[[["self"]],["t"]]],[11,"borrow_mut","","",5,[[["self"]],["t"]]],[11,"type_id","","",5,[[["self"]],["typeid"]]],[11,"receive_event","","",6,[[["self"],["domainevent"]]]],[11,"from","","",6,[[["t"]],["t"]]],[11,"into","","",6,[[],["u"]]],[11,"to_owned","","",6,[[["self"]],["t"]]],[11,"clone_into","","",6,[[["self"],["t"]]]],[11,"try_from","","",6,[[["u"]],["result"]]],[11,"try_into","","",6,[[],["result"]]],[11,"borrow","","",6,[[["self"]],["t"]]],[11,"borrow_mut","","",6,[[["self"]],["t"]]],[11,"type_id","","",6,[[["self"]],["typeid"]]],[11,"from","eventmill::command","",7,[[["t"]],["t"]]],[11,"into","","",7,[[],["u"]]],[11,"to_owned","","",7,[[["self"]],["t"]]],[11,"clone_into","","",7,[[["self"],["t"]]]],[11,"try_from","","",7,[[["u"]],["result"]]],[11,"try_into","","",7,[[],["result"]]],[11,"borrow","","",7,[[["self"]],["t"]]],[11,"borrow_mut","","",7,[[["self"]],["t"]]],[11,"type_id","","",7,[[["self"]],["typeid"]]],[11,"from","eventmill::dispatch","",13,[[["t"]],["t"]]],[11,"into","","",13,[[],["u"]]],[11,"try_from","","",13,[[["u"]],["result"]]],[11,"try_into","","",13,[[],["result"]]],[11,"borrow","","",13,[[["self"]],["t"]]],[11,"borrow_mut","","",13,[[["self"]],["t"]]],[11,"type_id","","",13,[[["self"]],["typeid"]]],[11,"from","","",9,[[["t"]],["t"]]],[11,"into","","",9,[[],["u"]]],[11,"to_string","","",9,[[["self"]],["string"]]],[11,"try_from","","",9,[[["u"]],["result"]]],[11,"try_into","","",9,[[],["result"]]],[11,"borrow","","",9,[[["self"]],["t"]]],[11,"borrow_mut","","",9,[[["self"]],["t"]]],[11,"type_id","","",9,[[["self"]],["typeid"]]],[11,"from","eventmill::event","",17,[[["t"]],["t"]]],[11,"into","","",17,[[],["u"]]],[11,"to_owned","","",17,[[["self"]],["t"]]],[11,"clone_into","","",17,[[["self"],["t"]]]],[11,"to_string","","",17,[[["self"]],["string"]]],[11,"try_from","","",17,[[["u"]],["result"]]],[11,"try_into","","",17,[[],["result"]]],[11,"borrow","","",17,[[["self"]],["t"]]],[11,"borrow_mut","","",17,[[["self"]],["t"]]],[11,"type_id","","",17,[[["self"]],["typeid"]]],[11,"from","","",14,[[["t"]],["t"]]],[11,"into","","",14,[[],["u"]]],[11,"to_owned","","",14,[[["self"]],["t"]]],[11,"clone_into","","",14,[[["self"],["t"]]]],[11,"try_from","","",14,[[["u"]],["result"]]],[11,"try_into","","",14,[[],["result"]]],[11,"borrow","","",14,[[["self"]],["t"]]],[11,"borrow_mut","","",14,[[["self"]],["t"]]],[11,"type_id","","",14,[[["self"]],["typeid"]]],[11,"from","","",15,[[["t"]],["t"]]],[11,"into","","",15,[[],["u"]]],[11,"to_owned","","",15,[[["self"]],["t"]]],[11,"clone_into","","",15,[[["self"],["t"]]]],[11,"try_from","","",15,[[["u"]],["result"]]],[11,"try_into","","",15,[[],["result"]]],[11,"borrow","","",15,[[["self"]],["t"]]],[11,"borrow_mut","","",15,[[["self"]],["t"]]],[11,"type_id","","",15,[[["self"]],["typeid"]]],[11,"from","eventmill::inmemory_store","",19,[[["t"]],["t"]]],[11,"into","","",19,[[],["u"]]],[11,"try_from","","",19,[[["u"]],["result"]]],[11,"try_into","","",19,[[],["result"]]],[11,"borrow","","",19,[[["self"]],["t"]]],[11,"borrow_mut","","",19,[[["self"]],["t"]]],[11,"type_id","","",19,[[["self"]],["typeid"]]],[11,"from","","",18,[[["t"]],["t"]]],[11,"into","","",18,[[],["u"]]],[11,"to_string","","",18,[[["self"]],["string"]]],[11,"try_from","","",18,[[["u"]],["result"]]],[11,"try_into","","",18,[[],["result"]]],[11,"borrow","","",18,[[["self"]],["t"]]],[11,"borrow_mut","","",18,[[["self"]],["t"]]],[11,"type_id","","",18,[[["self"]],["typeid"]]],[11,"from","eventmill::metadata","",20,[[["t"]],["t"]]],[11,"into","","",20,[[],["u"]]],[11,"to_owned","","",20,[[["self"]],["t"]]],[11,"clone_into","","",20,[[["self"],["t"]]]],[11,"to_string","","",20,[[["self"]],["string"]]],[11,"try_from","","",20,[[["u"]],["result"]]],[11,"try_into","","",20,[[],["result"]]],[11,"borrow","","",20,[[["self"]],["t"]]],[11,"borrow_mut","","",20,[[["self"]],["t"]]],[11,"type_id","","",20,[[["self"]],["typeid"]]],[11,"from","","",21,[[["t"]],["t"]]],[11,"into","","",21,[[],["u"]]],[11,"to_owned","","",21,[[["self"]],["t"]]],[11,"clone_into","","",21,[[["self"],["t"]]]],[11,"try_from","","",21,[[["u"]],["result"]]],[11,"try_into","","",21,[[],["result"]]],[11,"borrow","","",21,[[["self"]],["t"]]],[11,"borrow_mut","","",21,[[["self"]],["t"]]],[11,"type_id","","",21,[[["self"]],["typeid"]]],[11,"aggregate_id","eventmill::aggregate","",6,[[["self"]]]],[11,"generation","","",6,[[["self"]],["generation"]]],[11,"generation_mut","","",6,[[["self"]],["generation"]]],[11,"apply_event","","",6,[[["self"],["domainevent"]]]],[11,"initialize","","",6,[[["aggregateidof"]]]],[11,"handle_command","","",6,[[["c"],["self"]],[["result",["vec"]],["vec",["newevent"]]]]],[11,"dispatch_command","eventmill::dispatch","",13,[[["domaincommand"],["self"]],["result"]]],[11,"append","eventmill::inmemory_store","",19,[[["self"],["domainevent"]],["result"]]],[11,"append_batch","","",19,[[["self"]],["result"]]],[11,"read_events","","",19,[[["r"],["self"],["aggregateidof"]],["result"]]],[11,"from","eventmill::aggregate","",5,[[["sequence"]],["self"]]],[11,"from","eventmill::event","",17,[[["generation"]],["self"]]],[11,"from","","",15,[[],["self"]]],[11,"from","eventmill::metadata","",21,[[["string"]],["self"]]],[11,"from","","",21,[[["i32"]],["self"]]],[11,"from","","",21,[[["i64"]],["self"]]],[11,"from","","",21,[[["f32"]],["self"]]],[11,"from","","",21,[[["f64"]],["self"]]],[11,"from","","",21,[[["bool"]],["self"]]],[11,"from","","",21,[[["naivedate"]],["self"]]],[11,"from","","",21,[[["utc"],["datetime",["utc"]]],["self"]]],[11,"from","","",21,[[["value"],["vec",["value"]]],["self"]]],[11,"from","","",21,[[["value"],["hashmap",["string","value"]],["string"]],["self"]]],[11,"from","","",20,[[],["self"]]],[11,"clone","eventmill::aggregate","",5,[[["self"]],["generation"]]],[11,"clone","","",6,[[["self"]],["versionedaggregate"]]],[11,"clone","eventmill::command","",7,[[["self"]],["domaincommand"]]],[11,"clone","eventmill::event","",17,[[["self"]],["sequence"]]],[11,"clone","","",14,[[["self"]],["domainevent"]]],[11,"clone","","",15,[[["self"]],["newevent"]]],[11,"clone","eventmill::metadata","",21,[[["self"]],["value"]]],[11,"clone","","",20,[[["self"]],["attribute"]]],[11,"default","eventmill::aggregate","",5,[[],["self"]]],[11,"default","eventmill::event","",17,[[],["self"]]],[11,"default","eventmill::inmemory_store","",19,[[],["self"]]],[11,"cmp","eventmill::aggregate","",5,[[["generation"],["self"]],["ordering"]]],[11,"cmp","eventmill::event","",17,[[["sequence"],["self"]],["ordering"]]],[11,"eq","eventmill::aggregate","",5,[[["generation"],["self"]],["bool"]]],[11,"ne","","",5,[[["generation"],["self"]],["bool"]]],[11,"eq","","",6,[[["versionedaggregate"],["self"]],["bool"]]],[11,"ne","","",6,[[["versionedaggregate"],["self"]],["bool"]]],[11,"eq","eventmill::command","",7,[[["self"],["domaincommand"]],["bool"]]],[11,"ne","","",7,[[["self"],["domaincommand"]],["bool"]]],[11,"eq","eventmill::dispatch","",9,[[["self"],["coreerror"]],["bool"]]],[11,"ne","","",9,[[["self"],["coreerror"]],["bool"]]],[11,"eq","eventmill::event","",17,[[["sequence"],["self"]],["bool"]]],[11,"ne","","",17,[[["sequence"],["self"]],["bool"]]],[11,"eq","","",14,[[["domainevent"],["self"]],["bool"]]],[11,"ne","","",14,[[["domainevent"],["self"]],["bool"]]],[11,"eq","","",15,[[["newevent"],["self"]],["bool"]]],[11,"ne","","",15,[[["newevent"],["self"]],["bool"]]],[11,"eq","eventmill::inmemory_store","",18,[[["self"],["error"]],["bool"]]],[11,"ne","","",18,[[["self"],["error"]],["bool"]]],[11,"eq","eventmill::metadata","",21,[[["self"],["value"]],["bool"]]],[11,"ne","","",21,[[["self"],["value"]],["bool"]]],[11,"eq","","",20,[[["attribute"],["self"]],["bool"]]],[11,"ne","","",20,[[["attribute"],["self"]],["bool"]]],[11,"partial_cmp","eventmill::aggregate","",5,[[["generation"],["self"]],[["option",["ordering"]],["ordering"]]]],[11,"lt","","",5,[[["generation"],["self"]],["bool"]]],[11,"le","","",5,[[["generation"],["self"]],["bool"]]],[11,"gt","","",5,[[["generation"],["self"]],["bool"]]],[11,"ge","","",5,[[["generation"],["self"]],["bool"]]],[11,"partial_cmp","eventmill::event","",17,[[["sequence"],["self"]],[["option",["ordering"]],["ordering"]]]],[11,"lt","","",17,[[["sequence"],["self"]],["bool"]]],[11,"le","","",17,[[["sequence"],["self"]],["bool"]]],[11,"gt","","",17,[[["sequence"],["self"]],["bool"]]],[11,"ge","","",17,[[["sequence"],["self"]],["bool"]]],[11,"fmt","eventmill::aggregate","",5,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",6,[[["formatter"],["self"]],["result"]]],[11,"fmt","eventmill::command","",7,[[["formatter"],["self"]],["result"]]],[11,"fmt","eventmill::dispatch","",9,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",13,[[["formatter"],["self"]],["result"]]],[11,"fmt","eventmill::event","",17,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",14,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",15,[[["formatter"],["self"]],["result"]]],[11,"fmt","eventmill::inmemory_store","",18,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",19,[[["formatter"],["self"]],["result"]]],[11,"fmt","eventmill::metadata","",21,[[["formatter"],["self"]],["result"]]],[11,"fmt","","",20,[[["formatter"],["self"]],["result"]]],[11,"fmt","eventmill::aggregate","",5,[[["formatter"],["self"]],["result"]]],[11,"fmt","eventmill::dispatch","",9,[[["formatter"],["self"]],["result"]]],[11,"fmt","eventmill::event","",17,[[["formatter"],["self"]],["result"]]],[11,"fmt","eventmill::inmemory_store","",18,[[["formatter"],["self"]],["result"]]],[11,"fmt","eventmill::metadata","",20,[[["formatter"],["self"]],["result"]]],[11,"hash","eventmill::aggregate","",5,[[["self"],["__h"]]]],[11,"hash","eventmill::event","",17,[[["self"],["__h"]]]],[11,"serialize","eventmill::aggregate","",5,[[["self"],["__s"]],["result"]]],[11,"serialize","eventmill::command","",7,[[["self"],["__s"]],["result"]]],[11,"serialize","eventmill::dispatch","",9,[[["self"],["__s"]],["result"]]],[11,"serialize","eventmill::event","",17,[[["self"],["__s"]],["result"]]],[11,"serialize","","",14,[[["self"],["__s"]],["result"]]],[11,"serialize","","",15,[[["self"],["__s"]],["result"]]],[11,"serialize","eventmill::metadata","",21,[[["self"],["__s"]],["result"]]],[11,"serialize","","",20,[[["self"],["__s"]],["result"]]],[11,"deserialize","eventmill::aggregate","",5,[[["__d"]],["result"]]],[11,"deserialize","eventmill::command","",7,[[["__d"]],["result"]]],[11,"deserialize","eventmill::dispatch","",9,[[["__d"]],["result"]]],[11,"deserialize","eventmill::event","",17,[[["__d"]],["result"]]],[11,"deserialize","","",14,[[["__d"]],["result"]]],[11,"deserialize","","",15,[[["__d"]],["result"]]],[11,"deserialize","eventmill::metadata","",21,[[["__d"]],["result"]]],[11,"deserialize","","",20,[[["__d"]],["result"]]]],"p":[[8,"AggregateType"],[8,"WithAggregateId"],[8,"AggregateState"],[8,"Aggregate"],[8,"InitializeAggregate"],[3,"Generation"],[3,"VersionedAggregate"],[3,"DomainCommand"],[8,"HandleCommand"],[4,"CoreError"],[13,"GenerationConflict"],[8,"DispatchEvent"],[8,"DispatchCommand"],[3,"Core"],[3,"DomainEvent"],[3,"NewEvent"],[8,"EventType"],[3,"Sequence"],[4,"Error"],[3,"InMemoryStore"],[3,"Attribute"],[4,"Value"],[8,"ReceiveEvent"],[8,"EventSink"],[8,"EventSource"]]};
searchIndex["eventmill_derive"] = {"doc":"","i":[[24,"EventType","eventmill_derive","",null,null]],"p":[]};
addSearchOptions(searchIndex);initSearch(searchIndex);