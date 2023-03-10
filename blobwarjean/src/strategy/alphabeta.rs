//! Alpha - Beta algorithm.
use std::fmt;
use std::collections::HashMap;

use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;

/// Anytime alpha beta algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn alpha_beta_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    let mut transpo_table = HashMap::new();
    let mut root = node_init(state);
    for depth in 1..100 {
        /*let chosen_movement = AlphaBeta(depth).compute_next_move(state);
        movement.store(chosen_movement);*/
        tree_alpha_beta(root, transpo_table, -127, 127);
        movement.store(root.child.last().mov);
    }
}

//structure that intends to memorize the quality of the mouvements
struct Node{
    pos : &Configuration,
    childs : Vec<&Node>,
    eva : i8,
    mov : Option<Movement>
}

fn node_init(state : &Configuration) -> Node {
    res = Node {state, Vec::new(), 0};
    for movement in state.movements() {
        res.childs.push(Node {state.play(movement), Vec::new(), 0, movement});
    }
    return res;
}


// rajoute une couche de noeuds, et calcule leur valeur
fn expand_tree(mut parent: &Node, mut transpo_table: HashMap((u8,u8), Node),beta : i8) -> i8 {
    let mut pos;
    let mut res;
    for movement in parent.pos.movements() {
        pos = parent.pos.play(movement);
        if(transpo_table.contains_key(pos.getHash())){//on utilse la table de transposition
            parent.childs.push(transpo_table.get(pos.getHash()));
            res = parent.childs.last().eva;
        }
        else{
            res = pos.value;
            parent.childs.push(Node {pos , Vec::new(), res});
        }
        if(res >= beta){
            parent.childs.sort_unstable_by_key(|node| node.eva);
            return res;
        }
    }
    if !parent.childs.get(0) && !parent.pos.game_over(){
        pos = parent.pos.skip_play();
        res = pos.value();
        parent.childs.push(Node {pos, Vec::new(), res});
        return res;
    }
    parent.childs.sort_unstable_by_key(|node| node.eva);
    return parent.childs.last().eva;
}

//met à jour les évaluations de l'arbre, avec elagage ab et table de transpostion
//renvoie l'évaluation du noeud
//permet de s'émanciper de depth (car c'est fait implicitement : à chaque appel successif de la 
//racine on rajoute une couche de profondeur, c'est assez élégant)
fn tree_alpha_beta(mut root : &Node, mut transpo_table: HashMap((u8,u8), Node)
                    , mut alpha : i8, mut beta : i8) -> i8 {
    if !root.childs[0] {//cas feuille de l'arbre précédent
        return  - expand_tree(root, transpo_table, -alpha);
    }
    else{
        let mut value;
        for child in root.childs {
            value = tree_alpha_beta(root, transpo_table, -beta, -alpha);
            child.eva = value;
            if value >= beta {
                root.childs.sort_unstable_by_key(|node| node.eva);
                return -value;
            }
            if alpha < value {
                alpha = value;
            }
            
        }
    }
    root.childs.sort_unstable_by_key(|node| node.eva);
    return - root.childs.last().eva;
}

/// Alpha - Beta algorithm with given maximum number of recursions.
pub struct AlphaBeta(pub u8);

impl fmt::Display for AlphaBeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alpha - Beta (max level: {})", self.0)
    }
}

impl Strategy for AlphaBeta {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        return state.movements().max_by_key(|movement: &Movement| 
            nega_alpha_beta(self.0-1, &state.play(movement), -127, 127));
    }
}

fn nega_alpha_beta(depth: u8, state : &Configuration, mut alpha: i8, beta: i8) -> i8 {
    if depth == 0 || state.game_over(){
        return state.value();
    }
    else if state.movements().peekable().peek().is_none(){
        return -nega_alpha_beta(depth-1, &state.skip_play(), -beta, -alpha);
    }
    else{
        let mut best_value = -127;
        let mut value;
        for movement in state.movements() {
            value = nega_alpha_beta(depth - 1, &state.play(&movement), -beta, -alpha);
            if best_value < value {
                best_value = value;
                if best_value >= beta {
                    return -best_value;
                }
                if alpha < best_value {
                    alpha = best_value;
                }
            }
        }
        return -best_value;// si on gagne de 1, notre adversaire gagne de -1 aka il perd de 1
    }
}

