use chrono::NaiveDate;
use leptos::prelude::*;
use leptos_meta::*;

#[derive(Debug, Clone)]
enum Membership {
    Simple,
    Stander,
    Super,
}
#[allow(dead_code)]
impl Membership {
    fn new(membership_str: &str) -> Membership {
        match membership_str {
            "simple" => Membership::Simple,
            "stander" => Membership::Stander,
            "super" => Membership::Super,
            _ => panic!("Error create membership"),
        }
    }
}

#[derive(Debug, Clone)]
enum Gender {
    Male,
    Female,
}
#[allow(dead_code)]
impl Gender {
    fn new(gender_str: &str) -> Gender {
        match gender_str {
            "male" => Gender::Male,
            "female" => Gender::Female,
            _ => panic!("Gender Error"),
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct UserFormInput {
    name: String,
    email: String,
    message: String,
    gender: Gender,
    age: u8,
    birthdate: NaiveDate,
    membership: Membership,
    likes: Option<Vec<String>>,
}

#[allow(dead_code)]
impl UserFormInput {
    fn new(
        name: String,
        email: String,
        message: String,
        gender: Gender,
        age: u8,
        birthdate: NaiveDate,
        membership: Membership,
        likes: Option<Vec<String>>,
    ) -> UserFormInput {
        UserFormInput {
            name,
            email,
            message,
            gender,
            age,
            birthdate,
            membership,
            likes,
        }
    }
}

#[component]
pub fn Forms_input_fn() -> impl IntoView {
    let submit = ServerAction::<SubmitPerson>::new();

    view! {
        <Meta name="discription" content="forms, user input, registration" />
        <Title text="Forms & input" />
        <ActionForm action=submit>
            <div>
                <label for="name">"Name"</label>
                <br />
                <input
                    id="name"
                    name="person_name"
                    type="text"
                    placeholder="your name here"
                    required
                />
            </div>
            <div>
                <label for="email">"Email"</label>
                <br />
                <input
                    id="email"
                    name="person_email"
                    type="email"
                    placeholder="example@google.com"
                    required
                />
            </div>
            <div>
                <label for="message">"Message"</label>
                <br />
                <textarea
                    id="message"
                    name="person_message"
                    cols="30"
                    rows="10"
                    placeholder="Put your message here"
                ></textarea>
            </div>
            <div>
                <label for="sex">"Sex"</label>
                <select id="sex" name="person_gender" required>
                    <option value="male">"Male"</option>
                    <option value="female">"Female"</option>
                </select>
            </div>
            <div>
                <label for="age">"Age"</label>
                <input id="age" type="number" min="10" max="120" name="person_age" />
            </div>
            <div>
                <label for="birthdate">"Birthdate"</label>
                <br />
                <input id="birthdate" type="date" name="person_birthdate" required />
            </div>
            <div>
                <label for="membership">"Membership"</label>
                <input id="membership" type="radio" name="person_membership" value="simple" />
                Simple
                <input
                    id="membership"
                    type="radio"
                    name="person_membership"
                    value="stander"
                    checked
                />
                Stander
                <input id="membership" type="radio" name="person_membership" value="super" />
                Super
            </div>
            <div>
                <label for="likes">"I Like ..."</label>
                <input id="likes" type="checkbox" name="person_like[]" value="bike" />
                Bike
                <input id="likes" type="checkbox" name="person_like[]" value="car" />
                Car
                <input id="likes" type="checkbox" name="person_like[]" value="boat" />
                Boat
                <input id="likes" type="checkbox" name="person_like[]" value="programming" />
                Programming
            </div>
            <br />
            <input type="submit" value="Submit" />
            <button type="reset">"Clear"</button>
        </ActionForm>
    }
}

#[server]
async fn submit_person(
    person_name: String,
    person_email: String,
    person_message: String,
    person_gender: String,
    person_age: String,
    person_birthdate: String,
    person_membership: String,
    person_like: Option<Vec<String>>,
) -> Result<(), ServerFnError> {
    let date_time: NaiveDate = NaiveDate::parse_from_str(person_birthdate.as_str(), "%Y-%m-%d")
        .expect("Error parsing date");

    let gender: Gender = Gender::new(person_gender.as_str());
    let person_age: u8 = match person_age.trim().parse::<u8>() {
        Ok(val) => val,
        Err(_) => 0,
    };
    let membership = Membership::new(person_membership.as_str());

    let new_person: UserFormInput = UserFormInput::new(
        person_name,
        person_email,
        person_message,
        gender,
        person_age,
        date_time,
        membership,
        person_like,
    );

    println!("{:?}", new_person);

    match person_age {
        0 => leptos_actix::redirect("/not_found"),
        _ => {
            println!("{:?}", new_person);
            leptos_actix::redirect("/block_inlines")
        }
    }
    Ok(())
}
