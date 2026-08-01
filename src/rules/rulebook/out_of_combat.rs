use dioxus::prelude::*;

#[component]
pub fn OutOfCombatThread() -> Element {
  rsx! {
    div {
      class: "subtitle",
      "Out of Combat"
    }
    div {
      "Spending action points to represent what your character is doing second by second in combat is compelling and tactical, but many important scenes are played out over longer periods of time. When outside of combat instead of spending action points players assign action points. They can be assigned to either accomplish a task or to watch for a specific event. Characters can move at a relaxed pace and hold a simple conversation without needing to assign action points."
    }
    div {
      class: "underline",
      "Tasks"
    }
    div {
      "Tasks cover the majority of actions that a player could take from trying to convince a guard of their innocence or to making a sandwich. If the task is particularly complex a DM can require two action points be assigned, like when building a clockwork device or baking a souffle. Characters can also assign an extra action point to a task to gain advantage on any rolls related to the task. Some tasks might require a character to move at a steady pace, like carrying a fragile piece of art or looking for traps in the dungeon. In these cases you must move at half speed to perform the action (or move at full speed with a penalty set by the DM)."
    }
    div {
      class: "underline",
      "Watching"
    }
    div {
      "A character can also keep an eye out for a specific event, object or situation. They might try to determine if an npc is lying or if someone is following them. While watching for that event the character receives a +4 Defense bonus against any check made to hide from or deceive them or gains advantage on expertise checks made to understand details related to the situation. In some cases the DM can indicate that watching for a specific event would be complex, requiring two dice to be assigned. Such as searching for a trap or listening for a flaw in a legal defense."
    }
    div {
      class: "underline",
      "Context Switching"
    }
    div {
      "If the player wishes to, they can reassign how their action points are distributed. A player can reassign an action point in the middle of doing an action, but cannot then reassign it again for at least another minute. Trying to constantly switch between multiple tasks is difficult and the DM can rule that longer than a minute might be needed if a player is trying to switch between substantially different tasks."
    }
    div {
      class: "underline",
      "Flat Footed"
    }
    div {
      "If combat begins while a character has all of their action points assigned they will automatically be surprised. Unless that character was watching for and successfully noticed the event that triggered combat."
    }
  }
}
