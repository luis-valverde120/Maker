# Maker

A blazingly fast, data-driven universal project scaffolder built in Rust. 

Maker is not just another CLI tool; it is a universal template interpreter. Instead of hardcoding project structures, Maker relies on a highly scalable, centralized YAML configuration file. Rust acts purely as the execution engine, while the YAML file serves as the brain—dictating languages, frameworks, architectural patterns, and even editor configurations.

## The Philosophy: Data-Driven Design

Traditional project generators are rigid. If you want to add support for a new framework or modify an architectural pattern, you have to rewrite the source code and recompile the application. Maker solves this by completely decoupling the execution logic from the project templates.

* **Agnostic Engine:** The Rust core doesn't know what Python, TypeScript, Express, or FastAPI are. It simply parses rules and executes them.
* **Infinite Scalability:** Add new languages, frameworks, or dependencies just by updating the `config.yaml` file. No Rust compilation required.
* **Offline First:** The default configuration is seamlessly embedded directly into the binary, ensuring zero-latency project generation even without an internet connection.

## Core Features & Architecture

Maker uses a hierarchical context system within its YAML configuration to build projects dynamically:

### 1. Framework-Specific Dependency Management
Maker adapts to the ecosystem it is generating. Whether it needs to run `npm install` for a Node.js project or leverage `uv add` for a lightning-fast Python setup, Maker dynamically constructs the execution commands based on the YAML definitions.

### 2. Contextual Architectural Patterns
Define abstract patterns (like MVC, DDD, or Hexagonal) nested within specific frameworks. Maker maps these definitions to create the exact folder trees and auto-generate the base source code required for that specific environment.

### 3. Out-of-the-box Editor Standardization
Maker doesn't just create folders; it prepares the entire development environment. By defining `.vscode/settings.json` and `.vscode/extensions.json` in the configuration, every generated project is instantly ready for coding with standardized tab sizes, formatters, and tailored extensions for the team.

## The Brain: Configuration Example

Here is a glimpse of how Maker structures its rules dynamically via the configuration file:

### For configure for a framework 
In the file config.yaml we have a section for add or modify a framework

```YAML
  fastapi:
    language: python
    install_cmd: ["uv", "add"]
    install_dev_cmd: ["uv", "add", "--dev"]
    dependencies: ["fastapi", "uvicorn"]
    dev_dependencies: ["pytest", "ruff"]
    architectures:
      mvc:
        folders:
          - app/controllers
          - app/models
          - app/routes
          - app/templates
          - test
          - .vscode
        files:
          "app/main.py": |
            # Code initial
```

### modify dependencies for a project
This have a specific fields that to execute a command install_cmd this use for execute a package install for exmaple python using `uv`
And for add more dependencies or development dependencies we have 2 field that are `dependencies` and `dev_dependencies` for especifies a libraries to add in this project

```YAML
    install_cmd: ["uv", "add"]
    install_dev_cmd: ["uv", "add", "--dev"]
    dependencies: ["fastapi", "uvicorn", "SqlAlchemy", .., etc]
    dev_dependencies: ["pytest", "ruff"]
```

### Add another architectures
In the section to `architectures` we can add more architectures and modify the structure of folders of this 

```YAML
    architectures:
      mvc:
        folders:
          - app/controllers
          - app/models
          - app/routes
      hexagonal:
        folders:
          - internal/domain
          - internal/application
          - internal/ports
          - internal/adapters
```

