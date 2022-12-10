

pub struct SingleDiceRoll(u8);

trait Event{}
impl SingleDiceRoll for Event{}


pub struct AllPossibleDiceRolls;
impl SampleSpace for AllPossibleDiceRolls{
    type Item=SingleDiceRoll;
    type It=std::vec::IntoIter<Self::Item>;
    fn gen(self)->Self::It{
        (0u8..6).map(SingleDiceRoll).collect::<Vec<_>>().into_iter()
    }
}



pub trait Probability{
    
    type E;
    //Events associated with this outcome
    type It:Iterator<Item=Self::E>;

    //The total sample space
    type S:SampleSpace;

    fn get_events(&self)->Self::It;

    fn get_sample_space(&self)->Self::S;

    //Get the probability
    fn get(self)->f64;
}

pub struct Chain<A,B>{
    a:A,
    b:B
}

impl<A:Probability,B:Probability<E=A::E,S=A::S>> Probability for Chain<A,B>{
    type E=A::E;
    type S=A::S;
    type It=std::iter::Chain<A::It,B::It>;
    fn get_events()
}





pub struct AllEven;
impl RandomVariable for AllEven{
    type S=AllPossibleDiceRolls;
    type Item=usize;
    fn compute(s:Self::S)->usize{
        s.0 % 2 == 0
    }
}


SingleDiceRoll(2).and(SingleDiceRoll(4)).and(SingleDiceRoll(6))


pub struct ProbabilityAllEven{

}


trait SampleSpace{
    type Item:Event;
    type It:Iterator<Item=X>;
    fn gen(self)->Self::It;
}

trait RandomVariable{
    type S:Event;
    type Item;
    fn compute(s:Self::Item)->Item;
}


trait ProbabilityMass{
    type K:RandomVariable;
    fn compute_prob(self,a:Self::K)->f64;
}




fn main(){
    
    
}