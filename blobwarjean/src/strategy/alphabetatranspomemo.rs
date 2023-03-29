//! Alpha - Beta algorithm with transposition tables and memoristion of the heuristic of the best moves.
use std::fmt;
use std::collections::HashMap;
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;


/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn alpha_beta_transpo_memo_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    let mut root = init(*state);//we use the tree constructed on the previous iterations
    for _depth in 1..100 {
        let mut tp_cp = HashMap::<(u64, u64), i8>::new();//necessary to renew at every step
        let mut tp_op = HashMap::<(u64, u64), i8>::new();
        tree_alpha_beta(&mut root, &mut tp_cp, &mut tp_op, -127, 127);
        movement.store(root.childs.last().unwrap().mov);
    }
}

//structure that intends to memorize the quality of the mouvements
struct Node<'a>{
    pos : Configuration<'a>,
    childs : Vec<Node<'a>>,
    eva : i8,
    mov : Option<Movement>,
    rest : Vec<Node<'a>>
}

/*struct Eva {
    v : i8,
    mate : bool
}*/

impl Node<'_> {
    /*fn insert_into(&mut self, new_child : Node) {

    }

    fn reorder(&mut self){

    }*/    
}
/// 2 constructors
fn node(pos : Configuration, mov : Option<Movement>) -> Node {
    let mut i = Vec::new();
    for m in pos.movements() {
        i.push(m);
    }
    return Node {pos : pos, childs : Vec::new(), eva : pos.value(), mov : mov, rest : Vec::new()};
}

fn init(pos : Configuration) -> Node{
    let mut i = Vec::new();
    for m in pos.movements() {
        i.push(m);
    }
    return Node {pos : pos, childs : Vec::new(), eva : 0, mov : None, rest : Vec::new()};
}

// rajoute une couche de noeuds, et calcule leur valeur
fn expand_tree(parent: &mut Node<'_>, beta : i8) -> i8 {
    let mut pos;
    let mut res=0;
    for movement in parent.pos.movements() {
        pos = parent.pos.play(&movement);
        parent.rest.push(node(pos, Some(movement)));
    }
    if parent.rest.is_empty() && !parent.pos.game_over(){
        pos = parent.pos.skip_play();
        parent.childs.push(node(pos, None));
    }
    let n = parent.rest.len();
    for child in parent.rest.drain(0..n){
        res = child.eva;
        parent.childs.push(child);
        if res >= beta {
            parent.childs.sort_unstable_by_key(|node| -node.eva);
            return res;
        }
    }
    parent.childs.sort_unstable_by_key(|node| -node.eva);
    return res;
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
        let mut res = -64;
        let mut value;
        for child in (&mut root.childs).iter_mut() {
            //tables de transposition
            if tp_cp.contains_key(&child.pos.get_hash()){
                value = *(tp_cp.get(&child.pos.get_hash()).unwrap());
            }
            else{
                //l'appel recursif !
                value = -tree_alpha_beta(child, tp_op, tp_cp, -beta, -alpha);
                tp_cp.insert(child.pos.get_hash(), value);
            }
            //mise à jour de la valeur de l'enfant
            child.eva = value;
            if value >= beta {
                //coupure beta
                root.childs.sort_unstable_by_key(|node| -node.eva);
                return value;
            }
            if alpha < value {
                alpha = value;
            }
            if res < value{
                res = value;
            }
            
        }
        let n = root.rest.len();
        for mut child in (&mut root.rest).drain(0..n) {
            if tp_cp.contains_key(&child.pos.get_hash()){
                value = *(tp_cp.get(&child.pos.get_hash()).unwrap());
            }
            else{
                value = -tree_alpha_beta(&mut child, tp_op, tp_cp, -beta, -alpha);
                tp_cp.insert(child.pos.get_hash(), value);
            }
            child.eva = value;
            if value >= beta {
                root.childs.sort_unstable_by_key(|node| -node.eva);
                return -value;
            }
            if alpha < value {
                alpha = value;
            }
            if res < value{
                res = value;
            }
            root.childs.push(child);
            
        }
        root.childs.sort_unstable_by_key(|node| -node.eva);//à refaire
        return res;
    }
    
}

/// Alpha - Beta algorithm with given maximum number of recursions.
pub struct AlphaBetaTranspoMemo(pub u8);

impl fmt::Display for AlphaBetaTranspoMemo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alpha - Beta (max level: {})", self.0)
    }
}

impl Strategy for AlphaBetaTranspoMemo {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        let mut root = init(*state);// optimisation possible here
        for _depth in 0..self.0 {
            let mut tp_cp = HashMap::<(u64, u64), i8>::new();//necessary to renew at every step
            let mut tp_op = HashMap::<(u64, u64), i8>::new();
            tree_alpha_beta(&mut root, &mut tp_cp, &mut tp_op, -127, 127);
        }
        return root.childs.get(0).unwrap().mov;
    }
}