+++
title = "Leader Elections"
date = 2020-10-26T19:05:42-07:00
draft = false
gh_repo = "biome"

[menu]
  [menu.biome]
    title = "Leader Elections"
    identifier = "habitat/supervisors/Leader Elections"
    parent = "habitat/supervisors"
    weight = 40
+++

The Biome Supervisor performs leader election natively for service groups [topologies]({{< relref "service_group_topologies" >}}) that require one, such as _leader-follower_.

Because Biome is an eventually-consistent distributed system, the role of the leader is different than in strongly-consistent systems. It only serves as the leader for *application level semantics*, e.g. a database write leader. The fact that a Supervisor is a leader has no bearing upon other operations in the Biome system, including rumor dissemination for configuration updates. It is _not_ akin to a [Raft](https://raft.github.io/) leader, through which writes must all be funneled. This allows for very high scalability of the Biome Supervisor ring.

Services grouped using a leader need to have a minimum of three supervisors in order to break ties. It is also strongly recommended that you do not run the service group with an even number of members. Otherwise, in the event of a network partition with equal members on each side, both sides will elect a new leader, causing a full split-brain from which the algorithm cannot recover. Supervisors in a service group will warn you if you are using leader election and have an even number of supervisors.

### Protocol for electing a leader

When a service group starts in a leader topology, it will wait until there are sufficient members to form a quorum (at least three). At this point, an election cycle can happen. Each Supervisor injects an election rumor into ring, targeted at the service group, with the _exact same_ rumor, which demands an election and insists that the peer itself is the leader. This algorithm is known as [Bully](https://en.wikipedia.org/wiki/Bully_algorithm).

Every peer that receives this rumor does a simple lexicographic comparison of its GUID with the GUID of the peer contained in that rumor. The winner is the peer whose GUID is higher. The peer then adds a vote for the GUID of the winner, and shares the rumor with others, including the total number of votes of anyone who previously voted for this winner.

An election ends when a candidate peer X gets a rumor back from the ring saying that it (X) is the winner, with all members voting. At this point, it sends out a rumor saying it is the declared winner, and the election cycle ends.

### Related reading

* For more information about the Bully algorithm, see [Elections in a Distributed Computing System](http://dl.acm.org/citation.cfm?id=1309451) by Héctor García-Molina.
