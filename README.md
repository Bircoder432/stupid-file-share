StupidFileShare is my learning project for exploring how Axum works; I do not recommend using it in a production environment.

# Usage
- ### Upload one file: 
    ``` curl -F "file=@example.txt" http://localhost:3000/upload ```
- ### Upload multi files: 
    ``` curl -F "file=@example1.txt" -F "file=@example2.png" http://localhost:3000/upload ```
- ### List files per short: 
    ``` curl http://localhost:3000/abc123 ```
- ### Download file: 
    ``` curl -O http://localhost:3000/abc123/file1.txt ```
- ### Remove short via admin token: 
    ``` curl -X DELETE -H "Authorization: Bearer supersecretadmintoken" http://localhost:3000/admin/abc123 ```
