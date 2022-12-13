use yew::{function_component, html, Html, use_state, platform::spawn_local};

use crate::store::{user::UserStore, objects::user::User};

impl From<User> for Html{
    fn from(value: User) -> Self {

        html!{
            <tr>
                <td>{value.username}</td>
                <td>{value.email}</td>
                <td>{value.created_at}</td>
                <td>{value.plan.to_string()}</td>
                <td>{value.credits_used}</td>
                <td>{value.credits_remaining}</td>
            </tr>
        }
    }
}

#[function_component(ShowUsers)]
pub fn show_users() -> Html {
    let state = use_state(|| None);

    {
        let state = state.clone();
        if state.is_none(){
            spawn_local(async move {
                let new_state = match UserStore::get_users().await{
                    Ok(r) => Some(r),
                    Err(_) => None
                };
    
                state.set(new_state);
            })
        }
    }

    html!{
        {if let Some(users) = &*state{
            html!{
                    <table class="table table-dark">
                        <thead class="thead-dark">
                            <tr>
                                <th scope="col">{"Username"}</th>
                                <th scope="col">{"Email"}</th>
                                <th scope="col">{"Created at"}</th>
                                <th scope="col">{"Plan"}</th>
                                <th scope="col">{"Credits used"}</th>
                                <th scope="col">{"Credits remaining"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            {for users.items.iter().map(|u| Html::from(u.clone()))}
                        </tbody>
                    </table>
            }
        }else{
            html!{}
        }}
    }
}