#[derive(Debug, Clone)]
struct Friend {
    name: String,
}

#[derive(Debug)]
struct Group {
    name: String,
    friends: Vec<Friend>,
}

type DebtGroup = Vec<Debt>;

#[derive(Debug)]
struct Debt {
    friend: Friend,
    value: u32,
    is_paid: bool,
}

#[derive(Debug)]
struct Bill {
    service_name: String,
    group: Group,
    debt_group: DebtGroup,
}

impl Group {
    pub fn new(name: String) -> Self {
        Group {
            name,
            friends: vec![],
        }
    }

    pub fn add_friend(&mut self, friend: Friend) {
        self.friends.push(friend)
    }
}

impl Debt {
    pub fn new(friend: Friend, value: u32) -> Self {
        Self {
            friend,
            value,
            is_paid: false,
        }
    }
}

impl Bill {
    pub fn new(service_name: String, group: Group, amout: u32) -> Self {
        let friends_count = group.friends.len();
        let ammout_splitted: u32 = amout / friends_count as u32;
        let debt_group: DebtGroup = group
            .friends
            .iter()
            .map(|friend| Debt::new(friend.clone(), ammout_splitted))
            .collect::<DebtGroup>();

        Self {
            service_name,
            group,
            debt_group,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
