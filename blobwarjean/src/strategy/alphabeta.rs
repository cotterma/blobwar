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
    let mut tp_cp = HashMap::<(u64, u64), i8>::new();
    let mut tp_op = HashMap::<(u64, u64), i8>::new();
    let mut root = node(*state);
    root.node_init();
    for _depth in 1..100 {
        /*let chosen_movement = AlphaBeta(depth).compute_next_move(state);
        movement.store(chosen_movement);*/
        tree_alpha_beta(&mut root, &mut tp_cp, &mut tp_op, -127, 127);
        movement.store(root.childs.last().unwrap().mov);
    }
}

//structure that intends to memorize the quality of the mouvements
struct Node<'a>{
    pos : Configuration<'a>,
    childs : Vec<Node<'a>>,
    eva : i8,
    mov : Option<Movement>
}

impl Node<'_> {
    fn node_init<'a>(&mut self){
        for movement in self.pos.movements() {
            self.childs.push(Node {pos : self.pos.play(&movement), childs : Vec::new(), 
                                    eva : 0, mov : Some(movement)});
        }
    }
}

fn node(pos : Configuration) -> Node {
    return Node {pos : pos, childs : Vec::new(), eva : pos.value(), mov : None};
}


// rajoute une couche de noeuds, et calcule leur valeur
fn expand_tree(parent: &mut Node<'_>, beta : i8) -> i8 {
    let mut pos;
    let mut res;
    for movement in parent.pos.movements() {
        pos = parent.pos.play(&movement);
        parent.childs.push(node(pos));
        res = pos.value();
        if res >= beta {
            parent.childs.sort_unstable_by_key(|node| node.eva);
            return res;
        }
    }
    if parent.childs.is_empty() && !parent.pos.game_over(){
        pos = parent.pos.skip_play();
        res = pos.value();
        parent.childs.push(node(pos));
        return res;
    }
    parent.childs.sort_unstable_by_key(|node| node.eva);
    return parent.childs.last().unwrap().eva;
}

//met à jour les évaluations de l'arbre, avec elagage ab et tables de transpostion
//on utilise 2 tables, car chaque joueur en a une 
//renvoie l'évaluation du noeud
//permet de s'émanciper de depth (car c'est fait implicitement : à chaque appel successif de la 
//racine on rajoute une couche de profondeur, c'est assez élégant)
fn tree_alpha_beta(root : &mut Node<'_>, tp_cp: &mut HashMap<(u64, u64), i8>, tp_op: &mut HashMap<(u64,u64), i8>,
                    mut alpha : i8, beta : i8) -> i8 {
    if root.childs.is_empty() {//cas feuille de l'arbre précédent
        return  - expand_tree(root, -alpha);
    }
    else{
        let mut value;
        for child in (&mut root.childs).iter_mut() {
            if tp_cp.contains_key(&child.pos.get_hash()){
                value = *(tp_cp.get(&child.pos.get_hash()).unwrap());
            }
            else{
                value = tree_alpha_beta(child, tp_op, tp_cp, -beta, -alpha);
                tp_cp.insert(child.pos.get_hash(), value);
            }
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
    return - root.childs.last().unwrap().eva;
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

