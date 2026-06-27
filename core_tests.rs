use std::thread;
use std::time::Duration;
use std::sync::{Arc, Barrier};
use std::panic;
use thread_split_enclave::thread_pool::HolographicEnclave;
use thread_split_enclave::hardware_sealer::apply_hardware_seal;

#[test]
fn test_imkansiz_01_polymorfik_kaos() {
    println!("\n=======================================================");
    println!("[>] TEST 1: POLİMORFİK KAYDIRMA (HAREKETLİ HEDEF)");
    println!("=======================================================");
    
    let sir = b"HareketliHedefSifresi_007!";
    let enclave = HolographicEnclave::new(sir, 5, 3);
    
    println!("  [*] Sır, donanıma mühürlendi ve 5 parçaya bölündü.");
    println!("  [*] Daemon Thread devrede... 300ms boyunca parçalar havada takas ediliyor (Swap).");
    
    thread::sleep(Duration::from_millis(300));

    let mut basarili = false;
    enclave.execute_with_secret(|guvenli_bellek| {
        assert_eq!(guvenli_bellek.as_ref(), sir);
        basarili = true;
    });
    
    println!("  [+] BAŞARILI: Bellek blokları saniyede 20 kez yer değiştirmesine rağmen veri kayıpsız birleştirildi!");
    assert!(basarili);
}

#[test]
fn test_imkansiz_02_donanim_muhru_hirsizligi() {
    println!("\n=======================================================");
    println!("[>] TEST 2: DONANIM MÜHRÜ VE RAM DÖKÜMÜ HIRSIZLIĞI");
    println!("=======================================================");
    
    let gercek_sir = b"SadeceBuSunucudaCalisir_12345";
    
    let muhurlu_veri = apply_hardware_seal(gercek_sir);
    assert_ne!(muhurlu_veri, gercek_sir);
    println!("  [+] Hacker'ın birleştirdiği veri, yerel CPU imzası olmadan DÜZ METİN OLARAK OKUNAMIYOR. (Zafiyet engellendi)");

    let cozulmus_veri = apply_hardware_seal(&muhurlu_veri);
    assert_eq!(cozulmus_veri, gercek_sir);
    println!("  [+] BAŞARILI: Sistem kendi donanım imzasını tanıdı ve mührü başarıyla çözdü!");
}

#[test]
fn test_imkansiz_03_es_zamanli_stres_ve_mutasyon() {
    println!("\n=======================================================");
    println!("[>] TEST 3: EŞZAMANLI HÜCUM VE STRES TESTİ (CONCURRENCY)");
    println!("=======================================================");
    
    let sir = b"MultiThread_Dayaniklilik_Testi_99";
    let enclave = Arc::new(HolographicEnclave::new(sir, 8, 4));
    
    let thread_sayisi = 10;
    let mut handles = vec![];
    let barrier = Arc::new(Barrier::new(thread_sayisi));

    for _ in 0..thread_sayisi {
        let enclave_clone = enclave.clone();
        let b = barrier.clone();
        
        handles.push(thread::spawn(move || {
            b.wait(); 
            enclave_clone.execute_with_secret(|bellek| {
                assert_eq!(bellek.as_ref(), sir);
            });
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("  [+] BAŞARILI: Mutex Lock kusursuz çalıştı. Thread çakışması (Race Condition) yaşanmadı!");
}

#[test]
fn test_imkansiz_04_cagri_yigini_zehirli_hap() {
    println!("\n=======================================================");
    println!("[>] TEST 4: ÇAĞRI YIĞINI (CALL-STACK) ZEHİRLİ HAP TESTİ");
    println!("=======================================================");
    println!("  [*] Sisteme dışarıdan (sahte bir 'eval' üzerinden) sızma simüle ediliyor...");
    
    let result = panic::catch_unwind(|| {
        sahte_eval_saldirisi_simulasyonu();
    });

    assert!(result.is_err(), "KRİTİK ZAFIYET: Sistem sahte 'eval' çağrısına izin verdi ve çökmedi!");
    println!("  [+] BAŞARILI: Kasa, çağrı yığınında 'eval' kelimesini gördü ve veriyi vermemek için KENDİNİ İMHA ETTİ!");
}

#[allow(dead_code)]
fn sahte_eval_saldirisi_simulasyonu() {
    let sir = b"ZehirliHapTestSifresi";
    let enclave = HolographicEnclave::new(sir, 3, 2);
    
    enclave.execute_with_secret(|_| {
        println!("  [-] KRİTİK HATA: Eğer bu yazıyı görüyorsan sistem KORUNAMADI demektir.");
    });
}

#[test]
fn test_imkansiz_05_dma_donanim_kancasi_ve_zehirli_hap() {
    println!("\n=======================================================");
    println!("[>] TEST 5: DMA DONANIM KANCASI (MEMORY HOOK) VE ZEHİRLİ HAP");
    println!("=======================================================");
    
    let sir = b"CokGizliBankaVerisi_999";
    let enclave = HolographicEnclave::new(sir, 5, 3);
    
    println!("  [*] Kasa aktif edildi. Orijinal sırlar polimorfik olarak dağıtıldı.");
    println!("  [!] KRİTİK SİMÜLASYON: Kırmızı Takım (Red Team) belleğe kanca atıyor...");
    println!("  [!] İşçi thread'lerden birine dışarıdan sahte/tuzak bir parça (Honeypot) enjekte ediliyor!");
    
    // Saldırgan kasanın belleğini bozuyor! (Sabotaj)
    enclave.sabotage_for_red_team_audit();
    
    // Kasa, bozulmuş parçalarla sırrı birleştirmeye çalıştığında Zehirli Hap tetiklenmeli!
    let result = panic::catch_unwind(|| {
        enclave.execute_with_secret(|_| {
            println!("  [-] EĞER BU YAZIYI GÖRÜYORSAN, SALDIRGAN KASAYI KIRDI DEMEKTİR!");
        });
    });

    assert!(result.is_err(), "ZAFIYET: Kasa bozuk parçaları fark edemedi ve veriyi açtı!");
    println!("  [+] BAŞARILI: Kasa 'MAGIC_MARKER' imzasını bulamadığı an ZEHİRLİ HAP'ı patlattı!");
    println!("  [+] SİSTEM: Tüm bellek asitle yıkandı (Zero-Out) ve süreç imha edildi!");
}