//! UI language support. English is the default; other languages are partial
//! translations and fall back to English for untranslated strings.

use std::sync::Mutex;

/// Languages shown in the pickers, as (code, display name).
pub const LANGUAGES: [(&str, &str); 5] = [
    ("en", "English"),
    ("tr", "Türkçe"),
    ("de", "Deutsch"),
    ("es", "Español"),
    ("fr", "Français"),
];

static ACTIVE: Mutex<String> = Mutex::new(String::new());

/// Load the saved language (before sign-in, so the login screen can use it).
pub fn load() {
    let path = crate::data::base_dir().join("language.json");
    if let Ok(code) = std::fs::read_to_string(&path) {
        if let Ok(mut guard) = ACTIVE.lock() {
            *guard = code.trim().to_string();
        }
    }
}

pub fn current() -> String {
    ACTIVE.lock().ok().map(|g| g.clone()).unwrap_or_default()
}

/// Persist the language choice globally so it applies on the login screen too.
pub fn set(code: &str) {
    if let Ok(mut guard) = ACTIVE.lock() {
        *guard = code.to_string();
    }
    let _ = std::fs::create_dir_all(crate::data::base_dir());
    let _ = std::fs::write(crate::data::base_dir().join("language.json"), code);
}

pub fn name_of(code: &str) -> &'static str {
    LANGUAGES
        .iter()
        .find(|(c, _)| *c == code)
        .map(|(_, n)| *n)
        .unwrap_or("English")
}

/// Translate a UI string. Falls back to English when a language doesn't
/// provide the key.
pub fn t(code: &str, key: &str) -> String {
    match code {
        "tr" => tr(key),
        "de" => de(key),
        "es" => es(key),
        "fr" => fr(key),
        _ => key.to_string(),
    }
}

/// Keys are the English strings themselves, so `t(code, "Sign in")` returns
/// the English text when the language is English (or the key is missing).
fn tr(key: &str) -> String {
    match key {
        "Create a local profile to get started." => "Başlamak için yerel bir profil oluşturun.".to_string(),
        "Enter your password to continue." => "Devam etmek için şifrenizi girin.".to_string(),
        "Continue to your profile." => "Profilinize devam edin.".to_string(),
        "Choose your profile to continue." => "Devam etmek için profilinizi seçin.".to_string(),
        "everything stays on this device" => "her şey bu cihazda kalır".to_string(),
        "Sign in" => "Giriş yap".to_string(),
        "Continue" => "Devam et".to_string(),
        "Create profile" => "Profil oluştur".to_string(),
        "I already have a profile" => "Zaten bir profilim var".to_string(),
        "Create a new profile" => "Yeni profil oluştur".to_string(),
        "Profile name" => "Profil adı".to_string(),
        "Password" => "Şifre".to_string(),
        "Confirm password" => "Şifreyi tekrar".to_string(),
        "Password protected" => "Şifre korumalı".to_string(),
        "Click to sign in" => "Giriş yapmak için tıkla".to_string(),
        "Sign out" => "Çıkış yap".to_string(),
        "Delete account" => "Hesabı sil".to_string(),
        "Confirm delete" => "Silme işlemini onayla".to_string(),
        "Cancel" => "İptal".to_string(),
        "Language" => "Dil".to_string(),
        "Web search" => "Web araması".to_string(),
        "Type a message and press Enter to send ..." => "Bir mesaj yazın ve göndermek için Enter'a basın ...".to_string(),
        "Remove" => "Kaldır".to_string(),
        _ => key.to_string(),
    }
}

fn de(key: &str) -> String {
    match key {
        "Create a local profile to get started." => "Erstelle ein lokales Profil, um loszulegen.".to_string(),
        "Enter your password to continue." => "Gib dein Passwort ein, um fortzufahren.".to_string(),
        "Continue to your profile." => "Weiter zu deinem Profil.".to_string(),
        "Choose your profile to continue." => "Wähle dein Profil, um fortzufahren.".to_string(),
        "everything stays on this device" => "alles bleibt auf diesem Gerät".to_string(),
        "Sign in" => "Anmelden".to_string(),
        "Continue" => "Weiter".to_string(),
        "Create profile" => "Profil erstellen".to_string(),
        "I already have a profile" => "Ich habe bereits ein Profil".to_string(),
        "Create a new profile" => "Neues Profil erstellen".to_string(),
        "Profile name" => "Profilname".to_string(),
        "Password" => "Passwort".to_string(),
        "Confirm password" => "Passwort bestätigen".to_string(),
        "Password protected" => "Passwortgeschützt".to_string(),
        "Click to sign in" => "Zum Anmelden klicken".to_string(),
        "Sign out" => "Abmelden".to_string(),
        "Delete account" => "Konto löschen".to_string(),
        "Confirm delete" => "Löschen bestätigen".to_string(),
        "Cancel" => "Abbrechen".to_string(),
        "Language" => "Sprache".to_string(),
        "Web search" => "Websuche".to_string(),
        "Type a message and press Enter to send ..." => "Nachricht eingeben und mit Enter senden ...".to_string(),
        "Remove" => "Entfernen".to_string(),
        _ => key.to_string(),
    }
}

fn es(key: &str) -> String {
    match key {
        "Create a local profile to get started." => "Crea un perfil local para empezar.".to_string(),
        "Enter your password to continue." => "Introduce tu contraseña para continuar.".to_string(),
        "Continue to your profile." => "Continúa con tu perfil.".to_string(),
        "Choose your profile to continue." => "Elige tu perfil para continuar.".to_string(),
        "everything stays on this device" => "todo permanece en este dispositivo".to_string(),
        "Sign in" => "Iniciar sesión".to_string(),
        "Continue" => "Continuar".to_string(),
        "Create profile" => "Crear perfil".to_string(),
        "I already have a profile" => "Ya tengo un perfil".to_string(),
        "Create a new profile" => "Crear un nuevo perfil".to_string(),
        "Profile name" => "Nombre del perfil".to_string(),
        "Password" => "Contraseña".to_string(),
        "Confirm password" => "Confirmar contraseña".to_string(),
        "Password protected" => "Protegido con contraseña".to_string(),
        "Click to sign in" => "Haz clic para iniciar sesión".to_string(),
        "Sign out" => "Cerrar sesión".to_string(),
        "Delete account" => "Eliminar cuenta".to_string(),
        "Confirm delete" => "Confirmar eliminación".to_string(),
        "Cancel" => "Cancelar".to_string(),
        "Language" => "Idioma".to_string(),
        "Web search" => "Búsqueda web".to_string(),
        "Type a message and press Enter to send ..." => "Escribe un mensaje y pulsa Enter para enviar ...".to_string(),
        "Remove" => "Quitar".to_string(),
        _ => key.to_string(),
    }
}

fn fr(key: &str) -> String {
    match key {
        "Create a local profile to get started." => "Créez un profil local pour commencer.".to_string(),
        "Enter your password to continue." => "Saisissez votre mot de passe pour continuer.".to_string(),
        "Continue to your profile." => "Continuez vers votre profil.".to_string(),
        "Choose your profile to continue." => "Choisissez votre profil pour continuer.".to_string(),
        "everything stays on this device" => "tout reste sur cet appareil".to_string(),
        "Sign in" => "Se connecter".to_string(),
        "Continue" => "Continuer".to_string(),
        "Create profile" => "Créer un profil".to_string(),
        "I already have a profile" => "J'ai déjà un profil".to_string(),
        "Create a new profile" => "Créer un nouveau profil".to_string(),
        "Profile name" => "Nom du profil".to_string(),
        "Password" => "Mot de passe".to_string(),
        "Confirm password" => "Confirmer le mot de passe".to_string(),
        "Password protected" => "Protégé par mot de passe".to_string(),
        "Click to sign in" => "Cliquez pour vous connecter".to_string(),
        "Sign out" => "Se déconnecter".to_string(),
        "Delete account" => "Supprimer le compte".to_string(),
        "Confirm delete" => "Confirmer la suppression".to_string(),
        "Cancel" => "Annuler".to_string(),
        "Language" => "Langue".to_string(),
        "Web search" => "Recherche web".to_string(),
        "Type a message and press Enter to send ..." => "Tapez un message et appuyez sur Entrée pour envoyer ...".to_string(),
        "Remove" => "Retirer".to_string(),
        _ => key.to_string(),
    }
}
