# Requirement
There are users on the system. They have an indicator with the following:
- Green - Online
- Gray - Offline

Two components:
- User - int
- indicator - boolean

This system requires a key-value **access pattern**.
# Decision Decision
- Given a user ID, get the status of user. To have one API or more?
- Single API to get a single user. Another API to get status for multiple users

Reading is done. What about updating the data? How do we update the data?

There are 2 ways:
- Pull based
- Push based

Since we are using HTTP, we cannot use Pull based system. So, we have to send a heartbeat to the backend for database update.

Create an API `/heartbeat` to send to the server which makes the user active.

Question: Should we send it once when we login or periodically?
Since there is no way for us to know if someone is offline, we have to send it periodically.

If the server does not receive the heartbeat for a few intervals, the user is marked offline. This *few intervals* is a business logic and business decision.
# Keeping track of Heartbeat
Now, we also have to store when we received the last heartbeat of the person. So, our data now looks like:
- User ID
- Last Heartbeat

This changes the GET `/status` API. So,
- If no entry for user, then user is offline
- If there is entry, and last heartbeat < NOW() - 30 seconds, then offline
- else, online
# Scaling
- User ID - 4 bytes
- Heartbeat - 4 bytes

8 bytes per user. So, for 1 billion users = 8 GB. Therefore, storage is not a problem.

# Is it possible to optimize the storage?
Decision: For a user, they only care about online/offline. They don't need additional information. Therefore, it there is entry in DB, they are online, else they are offline.

Expire the entries when it exceeds 30s. How to do this?
## Approach 1 - CRON to delete entries
Create a CRON service that would delete all the entries. 
### Problems
- Need a Robust solution
- Managing a new CRON service
- Handle edge cases in business logic

Can we offload this to somewhere else?
## Approach 2 - Offload to other service
Requirements are now - Need key-value store and expiration. We can use:
- Redis
- DynamoDB

Have a TTL of 30s for each entry.

Which system to pick and why?

# How to handle millions of requests/sec
Every heartbeat requests results in 1 DB call. If 1m users, there will be 6m updates per minute.

Creating a connection is time consuming. 
Better is to have a connection pool. How many connection pools should we have? We can define minimum to maximum.

# Similar Use Cases
- Used to detect failure in a distributed system