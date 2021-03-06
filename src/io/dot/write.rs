use crate::graph::model::get_vertex_name;

pub fn write(models: crate::graph::model::Models) {
    for model in models.models {
        println!("digraph {} {{", model.name);
        
        for edge in model.edges {
            print!("  {} -> {} [label=\"{}", get_vertex_name(&model.vertices, &edge.source_vertex_id),
                                           get_vertex_name(&model.vertices, &edge.target_vertex_id),
                                           edge.name);
            if !edge.guard.is_empty() {
                print!("\\n[{}]", edge.guard);
            }
            if !edge.actions.is_empty() {
                for action in edge.actions {
                    print!("\\n{}", action);
                }
            }
            println!("\"]");
        }
        println!("}}");
    }
}