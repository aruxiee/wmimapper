
# 🗺️ wmimapper: Internal Endpoint Enumerator in Rust

High-performance network recon tool. Leverages WMI to perform multi-threaded dumps of system metadata across an internal network without requiring third-party agents on target machines.

⚠️ **Please Note:** This project is strictly for **Educational and Authorized Penetration Testing**. I am not responsible for any of the shenanigans you guys pull.

---

## 🔍 How Fingerprinting Happens
`wmimapper` does fingerprinting by querying hardware and software identifiers that are difficult to change.

*   **Hardware UUIDs:** Extracting `ProcessorID` from `Win32_Processor` provides a hardware signature.
*   **OS Build:** Capturing the exact `Version` (e.g. `10.0.26200`), we can map the target to specific patches and known vulnerabilities.
*   **Analysis:** Using `LastBootUpTime` allows an operator to determine how long a system has been active, which helps distinguish between high-uptime servers and frequently rebooted computers.
*   **Storage:** Querying `Win32_LogicalDisk` provides the exact file system and capacity, helping identify the machine's primary role (e.g. database server vs. standard laptop).

---

## 🏗 Why WMI?
Windows Management Instrumentation (WMI) is the infrastructure for management of data and operations on Windows. It is the preferred vector for this project because:
*   **Native Integration:** Every modern Windows machine has WMI built-in. No files need to be uploaded to target.
*   **Rich Data Source:** Acts as a master database for everything from CPU temperatures to installed security software.
*   **Remote Capability:** WMI is designed for remote administration, making it a way to pivot through a network.

---

## 🛡 Detection Evasion
While WMI is a great tool, running it against thousands of endpoints is loud. To operate effectively, this tool has:

*   **Encryption:** The script attempts to use encrypted RPC channels. This prevents network sniffers from seeing the actual data (like Serial Numbers) being exfiltrated across the wire.
*   **LotL:** Because `wmimapper` uses standard Windows protocols, it is harder to distinguish in an audit.
*   **Timing:** Defenses look for **Event ID 4624** (Logon) and **4662** (Operation performed). Operators should perform scans during busy work hours to hide in the noise of legitimate logins.

---

## 🛠 Instructions to Run

### Prerequisites
*   **Rust Toolchain:** Installed via [rustup.rs](https://rustup.rs).
*   **Target Subnet:** Make sure you have edited `main.rs` to reflect your actual internal subnet (e.g. `192.168.1`).

### Steps
- **Build Binary**
    ```bash
    cargo build --release
    ```
- **Execute**
    ```bash
    .\target\release\wmimapper.exe
    ```

---

## 💼 Use Cases & Impact
*   **Asset Inventory:** Rapidly map hardware on an internal network.
*   **Security Auditing:** Identify forgotten machines running outdated and vulnerable Windows versions.
*   **Post-Exploit Recon:** Allows for silent collection of high-value target data after initial access is gained.
*   **Impact:** Reduces the time required to map a 250-node subnet from hours to seconds.

---

## 📊 MITRE
| ID | Technique | Description |
| :--- | :--- | :--- |
| **T1047** | **Windows Management Instrumentation** | Using WMI to extract system information and execute commands. |
| **T1018** | **Remote System Discovery** | Identifying other hosts on the internal network. |
| **T1082** | **System Information Discovery** | Gathering OS, hardware, and version details for fingerprinting. |
| **T1046** | **Network Service Scanning** | Probing internal IP ranges for active RPC/WMI listeners. |

---

## 🚀 Wanna Improve It?
- **Output to File:** Modify the script to pipe `println!` results into a `.json` or `.csv` file for easier analysis in Excel or Splunk.
- **Cred Management:** Integrate the `with_credentials` method to allow the tool to pivot using compromised admin accounts across different domains.
- **Software Inventory:** Expand the structs to query `Win32_Product` to see every installed app on every machine.
- **Integration:** Link this tool with a port scanner to verify if Port 135 is open before attempting the WMI handshake.

---

<p align="center">
  With ❤️ by <b>Aradhya</b>
</p>
