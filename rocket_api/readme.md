## API Endpoints

**GET /count:**

Get the Count of Students

```sh
curl http://localhost:8000/count
```

**GET /groups/<group_size>**

Curl Command: Replace `<group_size>` with the desired number of members per group.

```sh
curl http://localhost:8000/groups/<group_size>
```

**POST /save_groups**

```sh
curl -X POST -H "Content-Type: application/json" -d @groups.json http://localhost:8000/save_groups
```

### Example Workflow

1. **Get the Count of Students:**
```sh
curl http://localhost:8000/count
```

2. **Generate Random Groups:**
```sh
curl http://localhost:8000/groups/3 -o groups.json
```

3. **Save Groups to the Database:**
```sh
curl -X POST -H "Content-Type: application/json" -d @groups.json http://localhost:8000/save_groups
```
