# ReachDB

ReachDB is a high-performance graph database implementation in Rust, designed for efficient storage and traversal of relationship-based data. It uses memory-mapped files for fast access to node and relationship records.

## Features

- **Memory-mapped storage**: Fast access to node and relationship data
- **Bidirectional relationships**: Each relationship connects source and target nodes
- **User-defined relationship types**: Custom relationship semantics through generics
- **Efficient traversals**: Iterators for relationship traversal
- **Persistent storage**: Data remains on disk between sessions

## Usage

Add ReachDB to your `Cargo.toml`:

```toml
[dependencies]
reachdb = { path = "path/to/reachdb" }
```

### Creating a Database

```rust
use reachdb::{Reachdb, UserDefinedRelationType, ReachdbError};

// Define your relationship types
#[derive(Debug)]
enum RelationType {
    IsA(u8),
    HasA(u8),
    DependsOn(u8)
}

// Implement the UserDefinedRelationType trait
impl UserDefinedRelationType for RelationType {
    fn get_type_id(relation: &str) -> Option<Self> {
        match relation {
            "IS-A" => Some(Self::IsA(0)),
            "HAS-A" => Some(Self::HasA(1)),
            "DEPENDS-ON" => Some(Self::DependsOn(2)),
            _ => None
        }
    }

    fn type_id(&self) -> u8 {
        match self {
            Self::IsA(id) => *id,
            Self::HasA(id) => *id,
            Self::DependsOn(id) => *id,
        }
    }

    fn get_type_str(id: u8) -> Option<String> {
        match id {
            0 => Some("IS-A".to_string()),
            1 => Some("HAS-A".to_string()),
            2 => Some("DEPENDS-ON".to_string()),
            _ => None
        }
    }
}

// Open or create a new database
let mut db = Reachdb::<RelationType>::open("data", Some(10000), Some(10000))?;
```

### Working with Nodes and Relationships

```rust
// Add edges (automatically creates nodes if they don't exist)
db.add_edge("Person", "Human", "IS-A")?;
db.add_edge("Person", "Arms", "HAS-A")?;
db.add_edge("Person", "Legs", "HAS-A")?;
db.add_edge("Application", "Database", "DEPENDS-ON")?;

// Get all relationships for a node
let node_id = 0; // Assuming "Person" has ID 0
let relationships = db.get_all_node_relations(node_id)?;

// Get only outgoing or incoming relationships
let outgoing = db.get_outgoing_node_relations(node_id)?;
let incoming = db.get_incoming_node_relations(node_id)?;

// Retrieve relationship details
for rel_id in relationships {
    let relation = db.get_relation(rel_id)?;
    println!("Source: {}, Target: {}, Type: {}", 
        db.get_property(relation.source_id)?,
        db.get_property(relation.target_id)?,
        RelationType::get_type_str(relation.type_id).unwrap_or("Unknown".to_string()));
}

// Find connected node through a relationship
let connected_node_id = db.get_connected_node(node_id, rel_id)?;
let connected_node_name = db.get_property(connected_node_id)?;

// Close the database (important for persisting metadata)
db.close()?;
```

## Core Components

### Records

ReachDB uses two primary record types:

1. **NodeRecord**: Stores node information and links to relationships
   ```rust
   struct NodeRecord {
       id: u64,
       first_relationship_id: u64,
       first_property_id: u64,
   }
   ```

2. **RelationshipRecord**: Stores relationship information with bidirectional links
   ```rust
   struct RelationshipRecord {
       source_id: u64,
       target_id: u64,
       type_id: u8,
       first_property_id: u64,
       next_src_relationship_id: u64,
       prev_src_relationship_id: u64,
       next_tgt_relationship_id: u64,
       prev_tgt_relationship_id: u64,
   }
   ```

### Relationship Navigation

The `RelationshipIterator` allows for traversing relationships in both directions:

```rust
// Iterate through all relationships for a node
let relationships = RelationshipRecord::into_iter(
    relation_mmap,
    &node_id,
    node.first_relationship_id
);

for rel_result in relationships {
    if let Ok((rel_id, relation)) = rel_result {
        // Process relationship
    }
}
```

## Implementation Details

- Uses memory-mapped files for fast persistence and retrieval
- Maintains bidirectional links between nodes and relationships
- Stores string properties separately from structural records
- Custom error handling for various failure modes
- Efficient relationship traversal using linked lists

## Performance Considerations

- Memory-mapped files provide near-native speed for data access
- Relationship chains allow for quick traversal without loading the entire graph
- String properties are stored once and referenced by ID
- Prefetch sizes can be customized based on expected graph size

## License

[Add your license information here]
