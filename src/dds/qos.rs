//use std::time::Duration;

use crate::dds::result::*;

// This is to be implemented by all DomanParticipant, Publisher, Subscriber, DataWriter, DataReader, Topic
pub trait HasQoSPolicy {
  fn get_qos<'a>(self) -> &'a QosPolicies;
  fn set_qos(self, new_qos: &QosPolicies) -> Result<()>;
}

// DDS spec 2.3.3 defines this as "long" with named constants from 0 to 22.
// numbering is from IDL PSM, but it should be unnecessary at the Rust application interface
#[derive(Copy,Clone,PartialEq,Eq,PartialOrd,Ord,Debug)]
pub enum QosPolicyId {
  //Invalid  // We should represent this using Option<QosPolicyId> where needed
  //UserData,  // 1
  Durability,  // 2
  Presentation,  // 3
  Deadline,
  LatencyBudget, // 5
  Ownership,
  //OwnershipStrength, // 7
  Liveliness,
  TimeBasedFilter, // 9
  //Partition,
  Reliability, // 11
  DestinationOrder,
  History, // 13
  ResourceLimits,
  //EntityFactory, // 15
  //WriterDataLifeCycle,
  //ReaderDataLifeCycle, // 17
  //TopicData, // 18
  //GroupData,
  //TransportPriority, // 20
  Lifespan,
  //DurabilityService, // 22
}



#[derive(Clone)]
pub struct QosPolicies {
  durability: Option<policy::Durability>,
  presentation: Option<policy::Presentation>,
  deadline: Option<policy::Deadline>,
  latency_budget: Option<policy::LatencyBudget>,
  ownership: Option<policy::Ownership>,
  liveliness: Option<policy::Liveliness>,
  time_based_filter: Option<policy::TimeBasedFilter>,
  relibility: Option<policy::Reliability>,
  destination_order: Option<policy::DestinationOrder>,
  history: Option<policy::History>,
  resource_limits: Option<policy::ResourceLimits>,
  lifespan: Option<policy::Lifespan>,
} 

// put these into a submodule to avoid repeating the word "policy" or "qospolicy"
pub mod policy {
  use std::time::Duration;
  /*
  pub struct UserData {
    pub value: Vec<u8>,
  }

  pub struct TopicData {
    pub value: Vec<u8>,
  }

  pub struct GropupData {
    pub value: Vec<u8>,
  }

  pub struct TransportPriority {
    pub value: i32,
  }
  */
  #[derive(Clone)]
  pub struct Lifespan {
    pub duration: Duration,
  }

  // this is a policy
  #[derive(Clone)]
  pub enum Durability {
    Volatile, TransientLocal, Transient, Persistent,
  }

  #[derive(Clone)]
  pub struct Presentation {
    pub access_scope: PresentationAccessScope,
    pub coherent_access: bool,
    pub ordered_access: bool,
  }

  // This is not an independent QoS Policy but a component of Presentation
  #[derive(Clone)]
  pub enum PresentationAccessScope {
    Instance, Topic, Group,
  }

  #[derive(Clone)]
  pub struct Deadline {
    pub period: Duration,
  }

  #[derive(Clone)]
  pub struct LatencyBudget {
    pub duration: Duration,
  }

  #[derive(Clone)]
  pub enum Ownership {
    Shared,
    Exclusive { strength: i32 }, // This also implements OwnershipStrength
  }

  #[derive(Clone)]
  pub struct Liveliness {
    kind: LivelinessKind,
    lease_duration: Duration,
  }

  #[derive(Clone)]
  pub enum LivelinessKind {
    Automatic, ManualByParticipant, ManulByTopic,
  }

  #[derive(Clone)]
  pub struct TimeBasedFilter {
    pub minimum_separation: Duration,
  }

  /*
  pub struct Partition {
    pub name: Vec<Vec<u8>>,
  }
  */
  #[derive(Clone)]
  pub enum Reliability {
    BestEffort,
    Reliable { max_blocking_time: Duration },
  }

  #[derive(Clone)]
  pub enum DestinationOrder {
    ByReceptionTimestamp,
    BySourceTimeStamp,
  }

  #[derive(Clone)]
  pub enum History {
    KeepLast { depth: i32 },
    KeepAll,
  }

  #[derive(Clone)]
  pub struct ResourceLimits {
    max_samples: i32,
    max_instances: i32,
    max_samples_per_instance: i32,
  }
  /*
  pub struct EntityFactory {
    autoenable_created_entities: bool,
  } 
  */
  // WriterDataLifecycle
  // ReaderDataLifeCycle

  // DurabilityService

}


// TODO: helper function to combine two QosPolicies: existing and modifications
// Described in 2.2.2.1.1.1 set_qos (abstract)

// TODO: helper function to check is a QosPolices object is inconsistent (by itself)

// TODO: helper function to check if two QosPolicies: Reequested and Offered are
// compatible, according to DDS spec 2.2.3