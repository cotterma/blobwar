//! Alpha - Beta algorithm with transposition tables and memorisation of the best moves (killermove).
use std::fmt;
use std::collections::HashMap;
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;


/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn alpha_beta_transpo_km_anytime(state: &Configuration) {
    //let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    let mut root = init(*state);//we use the tree constructed on the previous iterations
    for _depth in 1..100 {
        let mut tp_cp = HashMap::<(u64, u64), i8>::new();//necessary to renew at every step
        let mut tp_op = HashMap::<(u64, u64), i8>::new();
        tree_alpha_beta(&mut root, &mut tp_cp, &mut tp_op, -127, 127);
        //movement.store(root.childs.last().unwrap().mov);
    }
}

//structure that intends to memorize the quality of the mouvements
struct Node<'a>{
    pos : Configuration<'a>,
    childs : Vec<Node<'a>>,
    bestchilds : Vec<Node<'a>>,
    eva : i8,
    mov : Option<Movement>,
}


impl Node<'_> {
      
}
/// 2 constructors
fn node(pos : Configuration, mov : Option<Movement>) -> Node {
    return Node {pos : pos, childs : Vec::new(), bestchilds : Vec::new(), eva : pos.value(), mov : mov};
}

fn init(pos : Configuration) -> Node{
    return Node {pos : pos, childs : Vec::new(), bestchilds : Vec::new(), eva : 0, mov : None};
}

// rajoute une couche de noeuds, et calcule leur valeur
fn expand_tree(parent: &mut Node<'_>) -> i8 {
    let mut pos;
    let mut nn;
    let mut res = -127;
    for movement in parent.pos.movements() {
        pos = parent.pos.play(&movement);
        nn = node(pos, Some(movement));
        if nn.eva > res{
            res = nn.eva;
            parent.childs.append(&mut parent.bestchilds);
            parent.bestchilds.push(nn);
        }
        else if nn.eva == res{
            parent.bestchilds.push(nn);
        }
        
    }
    if parent.childs.is_empty() && !parent.pos.game_over(){
        pos = parent.pos.skip_play();
        res = pos.value();
        parent.childs.push(node(pos, None));
        return res;
    }
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
        return  - expand_tree(root);
    }
    else{
        let mut value;
        let mut bv = -127;
        let mut child;
        //let mut temp = Vec::new();
        for i in 0..(root.bestchilds.len()){
            child = root.bestchilds.get_mut(i).unwrap();
            if tp_cp.contains_key(&child.pos.get_hash()){
                value = *(tp_cp.get(&child.pos.get_hash()).unwrap());
            }
            else{
                value = tree_alpha_beta(child, tp_op, tp_cp, -beta, -alpha);
                tp_cp.insert(child.pos.get_hash(), value);
            }
            child.eva = value;
            if bv <= value{
                bv = value;
            }
            /*else{
                root.bestchilds.remove(i);
                temp.push(child);
            }*/
            if value >= beta {
                return -value;
            }
            if alpha < value {
                alpha = value;
            }
            
        }
        for i in  0..(root.childs.len()){
            child = root.bestchilds.get_mut(i).unwrap();
            if tp_cp.contains_key(&child.pos.get_hash()){
                value = *(tp_cp.get(&child.pos.get_hash()).unwrap());
            }
            else{
                value = tree_alpha_beta(child, tp_op, tp_cp, -beta, -alpha);
                tp_cp.insert(child.pos.get_hash(), value);
            }
            child.eva = value;
            if bv < value{
                bv = value;
                root.childs.append(&mut root.bestchilds);
                root.bestchilds.push(root.childs.remove(i));
            }
            if bv == value{
                root.bestchilds.push(root.childs.remove(i));
            }
            if value >= beta {
                return -value;
            }
            if alpha < value {
                alpha = value;
            }
            
        }
        //root.childs.append(temp);
        return - bv;
    }
}

/// Alpha - Beta algorithm with given maximum number of recursions.
pub struct AlphaBetaTranspoKM(pub u8);

impl fmt::Display for AlphaBetaTranspoKM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alpha - Beta (max level: {})", self.0)
    }
}

impl Strategy for AlphaBetaTranspoKM {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        let mut root = init(*state);// optimisation possible here
        for _depth in 1..self.0 {
            let mut tp_cp = HashMap::<(u64, u64), i8>::new();//necessary to renew at every step
            let mut tp_op = HashMap::<(u64, u64), i8>::new();
            tree_alpha_beta(&mut root, &mut tp_cp, &mut tp_op, -127, 127);
        }
        return root.bestchilds.get(0).unwrap().mov;
    }
}