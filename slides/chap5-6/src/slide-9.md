
### OWNERSHIP .......

When defining a struct we use ```String``` over ```&str``` because we want the instances to <i>own</i> all of its data.
Can have cases where struct references data owned by someone else but we need to know about <i>lifetimes</i> :/

We will find out more about this in section 10 (which feels like a <i>lifetime</i> away sadly...)

