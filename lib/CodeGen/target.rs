use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TargetArchitecture {
    X86, X86_64, ARM32, ARM64, RiscV32, RiscV64, Wasm32, Wasm64,
    Unknown(String),
}
impl TargetArchitecture {
    pub fn pointer_width(&self) -> u8 {
        match self {
            Self::X86 | Self::ARM32 | Self::RiscV32 | Self::Wasm32 => 32,
            _ => 64,
        }
    }
    pub fn name(&self) -> &str {
        match self {
            Self::X86 => "x86", Self::X86_64 => "x86-64",
            Self::ARM32 => "arm32", Self::ARM64 => "arm64",
            Self::RiscV32 => "riscv32", Self::RiscV64 => "riscv64",
            Self::Wasm32 => "wasm32", Self::Wasm64 => "wasm64",
            Self::Unknown(v) => v,
        }
    }
}
impl fmt::Display for TargetArchitecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.name()) }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TargetOperatingSystem {
    Linux, Windows, MacOS, FreeBSD, Android, IOS, Wasm, Unknown(String),
}
impl TargetOperatingSystem {
    pub fn name(&self) -> &str {
        match self {
            Self::Linux=>"linux", Self::Windows=>"windows", Self::MacOS=>"macos",
            Self::FreeBSD=>"freebsd", Self::Android=>"android", Self::IOS=>"ios",
            Self::Wasm=>"wasm", Self::Unknown(v)=>v,
        }
    }
}
impl fmt::Display for TargetOperatingSystem {
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{write!(f,"{}",self.name())}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TargetEnvironment { GNU, MSVC, Musl, EABI, EABIHF, Unknown(String) }
impl TargetEnvironment {
    pub fn name(&self)->&str {
        match self { Self::GNU=>"gnu",Self::MSVC=>"msvc",Self::Musl=>"musl",
            Self::EABI=>"eabi",Self::EABIHF=>"eabihf",Self::Unknown(v)=>v }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TargetTriple {
    pub architecture: TargetArchitecture,
    pub operating_system: TargetOperatingSystem,
    pub environment: Option<TargetEnvironment>,
}
impl TargetTriple {
    pub fn new(a:TargetArchitecture,o:TargetOperatingSystem,e:Option<TargetEnvironment>)->Self{
        Self{architecture:a,operating_system:o,environment:e}
    }
    pub fn host()->Self{
        let a=if cfg!(target_arch="x86"){TargetArchitecture::X86}
        else if cfg!(target_arch="x86_64"){TargetArchitecture::X86_64}
        else if cfg!(target_arch="arm"){TargetArchitecture::ARM32}
        else if cfg!(target_arch="aarch64"){TargetArchitecture::ARM64}
        else if cfg!(target_arch="riscv32"){TargetArchitecture::RiscV32}
        else if cfg!(target_arch="riscv64"){TargetArchitecture::RiscV64}
        else{TargetArchitecture::Unknown(std::env::consts::ARCH.into())};
        let o=if cfg!(target_os="linux"){TargetOperatingSystem::Linux}
        else if cfg!(target_os="windows"){TargetOperatingSystem::Windows}
        else if cfg!(target_os="macos"){TargetOperatingSystem::MacOS}
        else if cfg!(target_os="freebsd"){TargetOperatingSystem::FreeBSD}
        else if cfg!(target_os="android"){TargetOperatingSystem::Android}
        else if cfg!(target_os="ios"){TargetOperatingSystem::IOS}
        else{TargetOperatingSystem::Unknown(std::env::consts::OS.into())};
        Self::new(a,o,None)
    }
    pub fn triple_string(&self)->String{
        match &self.environment {
            Some(e)=>format!("{}-{}-{}",self.architecture,self.operating_system,e.name()),
            None=>format!("{}-{}",self.architecture,self.operating_system)
        }
    }
}
