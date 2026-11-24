use std::mem::zeroed;
use std::ptr::null_mut;

use windows_sys::Win32::Foundation::{CloseHandle, LocalFree, FALSE, HANDLE, HLOCAL, PSID};
use windows_sys::Win32::Security::Authorization::{
    SetEntriesInAclW, SetSecurityInfo, DENY_ACCESS, EXPLICIT_ACCESS_W, NO_MULTIPLE_TRUSTEE,
    SE_KERNEL_OBJECT, TRUSTEE_IS_SID, TRUSTEE_IS_USER, TRUSTEE_W,
};
use windows_sys::Win32::Security::{
    GetTokenInformation, TokenUser, ACL, DACL_SECURITY_INFORMATION, NO_INHERITANCE,
    PROTECTED_DACL_SECURITY_INFORMATION, TOKEN_QUERY, TOKEN_USER,
};
use windows_sys::Win32::System::Threading::{
    GetCurrentProcess, OpenProcessToken, PROCESS_CREATE_THREAD, PROCESS_DUP_HANDLE,
    PROCESS_SET_INFORMATION, PROCESS_SET_QUOTA, PROCESS_VM_OPERATION, PROCESS_VM_READ,
    PROCESS_VM_WRITE,
};

// Constantes Windows standard pour WRITE_DAC et WRITE_OWNER
const WRITE_DAC: u32 = 0x00040000;
const WRITE_OWNER: u32 = 0x00080000;

pub fn securiser_processus_dacl() -> Result<(), String> {
    unsafe {
        let current_process: HANDLE = GetCurrentProcess();
        let mut token_handle: HANDLE = 0;

        // 1. Ouverture du jeton (Token) du processus courant pour récupérer le SID de l'utilisateur
        if OpenProcessToken(current_process, TOKEN_QUERY, &mut token_handle) == FALSE {
            return Err("Impossible d'ouvrir le token du processus courant.".to_string());
        }

        // 2. Récupération de la taille nécessaire pour les informations TokenUser
        let mut return_length: u32 = 0;
        GetTokenInformation(token_handle, TokenUser, null_mut(), 0, &mut return_length);

        if return_length == 0 {
            CloseHandle(token_handle);
            return Err("Impossible de déterminer la taille du TokenUser.".to_string());
        }

        // Allocation du buffer pour stocker TOKEN_USER
        let mut buffer = vec![0u8; return_length as usize];
        let token_user_ptr = buffer.as_mut_ptr() as *mut TOKEN_USER;

        if GetTokenInformation(
            token_handle,
            TokenUser,
            token_user_ptr as *mut _,
            return_length,
            &mut return_length,
        ) == FALSE
        {
            CloseHandle(token_handle);
            return Err("Échec de GetTokenInformation.".to_string());
        }

        let user_sid: PSID = (*token_user_ptr).User.Sid;

        // 3. Définition des droits à BLOQUER (DENY_ACCESS) sur ce processus
        let droits_bloques: u32 = PROCESS_VM_READ
            | PROCESS_VM_WRITE
            | PROCESS_VM_OPERATION
            | PROCESS_CREATE_THREAD
            | PROCESS_DUP_HANDLE
            | PROCESS_SET_INFORMATION
            | PROCESS_SET_QUOTA
            | WRITE_DAC
            | WRITE_OWNER;

        let mut ea: EXPLICIT_ACCESS_W = zeroed();
        ea.grfAccessPermissions = droits_bloques;
        ea.grfAccessMode = DENY_ACCESS;
        ea.grfInheritance = NO_INHERITANCE;
        ea.Trustee = TRUSTEE_W {
            pMultipleTrustee: null_mut(),
            MultipleTrusteeOperation: NO_MULTIPLE_TRUSTEE,
            TrusteeForm: TRUSTEE_IS_SID,
            TrusteeType: TRUSTEE_IS_USER,
            ptstrName: user_sid as *mut u16,
        };

        // 4. Création de la nouvelle ACL contenant notre règle de blocage
        let mut new_dacl: *mut ACL = null_mut();
        let acl_result = SetEntriesInAclW(1, &ea, null_mut(), &mut new_dacl);

        if acl_result != 0 {
            CloseHandle(token_handle);
            return Err(format!("Échec de SetEntriesInAclW (Code erreur: {})", acl_result));
        }

        // 5. Application de la DACL protégée sur le processus courant
        let sec_result = SetSecurityInfo(
            current_process,
            SE_KERNEL_OBJECT,
            DACL_SECURITY_INFORMATION | PROTECTED_DACL_SECURITY_INFORMATION,
            null_mut(),
            null_mut(),
            new_dacl,
            null_mut(),
        );

        // Nettoyage de la mémoire allouée par l'API Windows
        if !new_dacl.is_null() {
            LocalFree(new_dacl as HLOCAL);
        }
        CloseHandle(token_handle);

        if sec_result != 0 {
            return Err(format!("Échec de SetSecurityInfo (Code erreur: {})", sec_result));
        }

        Ok(())
    }
}