
# Quatuomotron (Vue.js 3 + Rust)

This student grouping application allows users to manage a list of students and generate random groups of 2 or 3 students. It consists of a Vue.js front-end and a Rust back-end.

Its the world famous Binomotron app made popular by the legendary Stephane, but re_imagined in Rust by 4 degenerates (in a single 4 hour afternoon coding session) hence the name Quatuomotron.

## Technologies Used

- **Front-end:** Vue.js 3
- **Back-end:** Rust
- **Build and Development:** Vite
- **Navigation:** Vue Router

## Project Structure

- **Front-end:** Located in the `vue` directory, built with Vue.js.
- **Back-end:** Located in the `rocket` directory, built with Rust.

## API Endpoints

### **GET /count:**

Get the Count of Students

```sh
curl http://localhost:8000/count
```

### **GET /groups/<group_size>**

Curl Command: Replace `<group_size>` with the desired number of members per group.

```sh
curl http://localhost:8000/groups/<group_size>
```

### **POST /save_groups**

Save the generated groups to the database.

```sh
curl -X POST -H "Content-Type: application/json" -d @groups.json http://localhost:8000/save_groups
```

## Example Workflow

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


## Front-end Setup

1. Navigate to the `vue` directory:
    ```sh
    cd vue
    ```

2. Install dependencies:
    ```sh
    npm install
    ```

3. Run the development server:
    ```sh
    npm run dev
    ```

4. Navigate to `http://localhost:5173/` in your browser.

## Back-end Setup

1. Navigate to the `rocket` directory:
    ```sh
    cd rocket
    ```

2. Build the Rust project:
    ```sh
    cargo build
    ```

3. Run the server:
    ```sh
    cargo run
    ```

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request.

## License

This project is licensed under the "if you like it, then it's yours" License.

