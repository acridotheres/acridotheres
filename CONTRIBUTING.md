# Contributing  

Thank you for your interest in contributing to Acridotheres! This guide will help you understand the structure of the project and how to get started with contributing.  

---

## Repositories  

This project has been divided into several repositories to streamline development and ensure modularity. Below is an overview of the repositories and their roles:  

### Base Functionality  

#### Shared Code
The shared code, including data types, compression, encryption, and hashing algorithms, is managed through the [acr](https://github.com/acridotheres/acr) repository.  

- Core utilities required for all file format implementations.  
- Updates here can affect multiple repositories, so extensive testing is required.  

#### Data Manipulation
Binary file reading and editing is handled by the [dh](https://github.com/Le0X8/dh) project.  

- Efficiently processes large files without loading them entirely into memory.  
- Offers functionality such as variable-length integer handling, bit-level manipulations, and high-performance binary I/O.  
- All file format implementations depend on this crate.

### File Formats  

Each supported file format is implemented in its own repository for modularity:  

- **HSSP 7**: [HSSP7](https://github.com/acridotheres/hssp7)  
- **ZIP**: [NeoZip](https://github.com/acridotheres/neozip)  
- **RAR**: [NeoRar](https://github.com/acridotheres/neorar)  
- **HSSP 6**: [HSSP6](https://github.com/acridotheres/hssp6)  
- **HSSP 5 & 4**: [HSSP5](https://github.com/acridotheres/hssp5)  
- **UMSBT & MSBT**: [3ds-formats](https://github.com/acridotheres/3ds-formats)  
- **HSSP 3, 2, and SFA**: [HSSP2](https://github.com/acridotheres/hssp2)  

To contribute to a specific file format, consult its repository for documentation, open issues, and development guidelines.  

## How to Contribute  

### 1. Reporting Bugs  
If you encounter a bug, please:  
1. Identify the relevant repository (e.g., `acr` for core functionality or `neozip` for ZIP-related issues).  
2. [Open an issue](https://github.com/acridotheres/acridotheres/issues/new) and provide:  
   - A clear and concise description of the problem.  
   - Steps to reproduce the issue.  
   - Relevant logs, screenshots, or example files.  

### 2. Requesting Features 
We welcome new feature ideas!  
- To propose a feature, [open a feature request issue](https://github.com/acridotheres/acridotheres/issues/new).  
- Include as much detail as possible, including the problem you're trying to solve, potential use cases, and implementation ideas.  

### 3. Code Contributions  

#### Step 1: Fork and Clone 
- Fork the relevant repository and clone it locally.  

#### Step 2: Set Up Your Development Environment  
- Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.  
- Install dependencies by running:  
  ```bash
  cargo build
  ```  

#### Step 3: Create a Feature Branch  
- Use a descriptive branch name (e.g., `fix-hssp7-parsing`, `feature-zip-support`).  

#### Step 4: Make Changes
- Follow the coding standards outlined in the repository.  
- Write or update tests to cover your changes.  

#### Step 5: Test Your Changes
- Run all tests locally:  
  ```bash
  cargo test
  ```  

#### Step 6: Submit a Pull Request
- Open a PR to the repository's `main` or `dev` branch (smaller ones use `main`, larger ones are divided into `dev`, `alpha`, `beta`, `rc` and `main`).
- Provide a clear description of your changes and reference any related issues.  

## Development Workflow  

### Testing
All repositories have automated tests to ensure stability and correctness. Before submitting a pull request, make sure:  
1. All existing tests pass.  
2. New tests are added for any functionality you implement.  

To run tests:  
```bash
cargo test
```  

For advanced testing (e.g., performance benchmarks or memory profiling), refer to the repository-specific documentation.  

### Coding Style
- Use [rustfmt](https://github.com/rust-lang/rustfmt) for consistent formatting:  
  ```bash
  cargo fmt
  ```  
- Check for common issues and improvements using [clippy](https://github.com/rust-lang/rust-clippy):  
  ```bash
  cargo clippy
  ```  

## Guidelines for File Format Contributions  

If you'd like to add support for a new file format:  
1. **Check for Duplicates**: Ensure no one else is already working on it.  
2. **Create a New Repository**: Follow the existing repositories' structure as a guide.  
3. **Use `acr` and `dh`:** Build on top of the shared code and data manipulation libraries.  
4. **Testing**: Include real-world examples (if legally permissible) and edge cases for testing.  
5. **Documentation**: Clearly document how the format is implemented, including any quirks or limitations.  

For guidance, refer to repositories like [HSSP7](https://github.com/acridotheres/hssp7) or [NeoZip](https://github.com/acridotheres/neozip).  

## Need Help?  

If you have any questions or need help getting started:  
- Join the discussion on [GitHub Discussions](https://github.com/acridotheres/acridotheres/discussions).  
- Open an issue with the "question" label.  
- Reach out to the maintainers directly via [GitHub](https://github.com/Le0X8).  

---

We’re excited to have you as a contributor to Acridotheres! Your contributions, whether big or small, make a difference. 😊  
