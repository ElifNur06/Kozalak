# 🛡️ KOZALAK (Holographic Memory Vault)

**Kozalak**, modern backend sistemlerinde verilerin RAM üzerinde statik ve "düz metin" (plaintext) olarak tutulması zafiyetine karşı geliştirilmiş, proaktif bir siber savunma kütüphanesidir. 

Geleneksel şifreleme yöntemleri statiktir; veriyi şifreler ve bellekte bir "kasa" içinde durağan bir şekilde saklar. **Kozalak ise aktif bir savunma mekanizmasıdır; veriyi 'yaşayan', hareket eden ve tehlike anında kendini imha edebilen bir yapıya dönüştürür.**

---

## 🏗️ Mimari Şema
## 🛠️ Temel Doğrulama (Unit Tests)

Kozalak mimarisi, modüler yapısı sayesinde her bir güvenlik katmanı için izole edilmiş testlere sahiptir. Aşağıdaki test sonuçları, projenin temel kriptografik ve bellek yönetim fonksiyonlarının (Zero-Out, Shamir parçalama ve Enclave yaşam döngüsü) %100 doğrulukla çalıştığını kanıtlamaktadır.

<img width="1252" height="704" alt="image" src="https://github.com/user-attachments/assets/0f1bc6d4-2058-40ca-86d3-daeb91c3e5e7" />

## 🚀 Python ile Güvenli Entegrasyon (Canlı Test)

Kozalak, yüksek seviyeli dillerde (Python) bile bellek seviyesinde tam kontrol sağlar. Aşağıdaki test çıktısı, verinin Python belleğinde hiçbir şekilde iz bırakmadan Rust katmanında işlenip, işlemin hemen ardından RAM'den tamamen silindiğini doğrulamaktadır.

<img width="1254" height="705" alt="image" src="https://github.com/user-attachments/assets/a6b5d7b1-93a4-4e02-b909-917adff4e978" />

> "Python'un esnekliği, Rust'ın sarsılmaz güvenliği ile birleşti."

## ⚖️ Stres Testi ve Kararlılık (Stress & Concurrency)

Kozalak, yüksek trafikli backend ortamlarında (örneğin Yankı BaaS) kararlılığını kanıtlamıştır. Eşzamanlı (multi-threaded) saldırı simülasyonlarında, sistem kaynaklı hiçbir bellek çakışması veya veri tutarsızlığı yaşanmamıştır.

<img width="1253" height="704" alt="image" src="https://github.com/user-attachments/assets/1f46f590-1564-4712-b86b-7952cd773dd7" />

> "Kozalak, sadece saldırganlara karşı değil, aynı zamanda yüksek sistem yüküne karşı da tam bir sarsılmazlık sunar."

## 🕵️ Proaktif Tehdit Algılama (Runtime Security)

Kozalak, çalışma zamanındaki kod davranışlarını (call-stack) denetleyerek RCE (Remote Code Execution) saldırılarını henüz tetiklenmeden durdurur. Geleneksel güvenlik duvarlarının (WAF) yakalayamadığı "eval" ve "exec" tabanlı sızma girişimlerini, kütüphane çekirdeği anında tespit eder ve veriyi korumak için sistemi acil durum modunda kapatır.

<img width="1253" height="704" alt="image" src="https://github.com/user-attachments/assets/a6f7fa64-f1f7-4f46-ac79-5e8f42d08a10" />

> "Kozalak, kodunuzun sadece niyetini değil, davranışını da denetler."

## 🌐 Edge-Enclave (WebAssembly) Desteği

Kozalak, sunucu tarafındaki sarsılmaz güvenliğini, tarayıcı ortamına (Client-Side) taşır. WASM'a derlenen kütüphanemiz, tarayıcı belleğindeki hassas verileri XSS (Cross-Site Scripting) saldırılarına karşı korumak için tasarlanmıştır. Sadece 560 KB olan modül boyutuyla, yüksek performanslı ve "kendi kendini imha eden" bir ön-uç (frontend) güvenliği sağlar.

<img width="1253" height="704" alt="image" src="https://github.com/user-attachments/assets/cf73167b-ef82-40cc-83a9-03cb5e96f0b4" />

> "Güvenlik, sunucuda başlayıp tarayıcının en ucunda (edge) bitmelidir."

## 🎯 Kırmızı Takım (Red Team) Sızma Testleri

Kozalak, belleğe doğrudan "kanca atan" (DMA Memory Hooking) ve manipülasyon yapmaya çalışan sızma girişimlerini tespit eder. Saldırı anında sistem, veriyi korumakla kalmaz, saldırganı etkisiz hale getirmek için **Zehirli Hap (Poison Pill)** protokolünü başlatarak tüm bellek yapısını asitle yıkar (Zero-Out).

<img width="1251" height="704" alt="image" src="https://github.com/user-attachments/assets/203839a4-3dcf-4b89-8cf8-ba6f6742e23e" />

> "Bellek manipüle edildiği milisaniyede sistemin imhası, verinin güvenliğinden daha önceliklidir."

## 🛠️ Pratik Uygulama: Yankı BaaS Senaryosu

Kozalak, gerçek dünya backend uygulamalarında (FastAPI/Yankı BaaS) veritabanı şifrelerini ve hassas anahtarları korumak için tasarlanmıştır. Veri, sadece bağlantı işlemi gerçekleştiği "mikro-an" süresince çözülür ve işlem bittiği anda bellekten tamamen silinerek hiçbir iz bırakmaz.

<img width="1252" height="707" alt="image" src="https://github.com/user-attachments/assets/152ce015-1fbc-4db8-8e55-860185ae63a9" />

> "Bellekte sır bırakmayan backend mimarisi, gerçek güvenliğin başlangıcıdır."

---

## 🚀 Derinlemesine Özellikler

### 1. Donanım Mührü (Hardware Sealing)
Kozalak, veriyi işlemci (CPU) özgün kimliğiyle (Unique CPU ID) mühürler. RAM dökümü (Memory Dump) alınsa bile, elde edilen veri başka bir makinede veya işlemcide donanımsal imza eşleşmeyeceği için hiçbir anlam ifade etmez.

### 2. Polimorfik Veri Kaydırma (Moving Target Defense)
Veri, *Shamir's Secret Sharing* algoritması ile parçalara (shares) bölünür. Arka planda çalışan "Kozalak Daemon", bu parçaları RAM adresleri arasında saniyede 20 kez takas eder (Swap). Bu dinamik yapı sayesinde;
- Statik bellek analizi araçları (GDB, Cheat Engine) geçersiz kalır.
- Veri, bellek üzerinde "hareketli bir hedef" haline gelir.

### 3. Zehirli Hap (Poison Pill) & Asitle Yıkama
Bellek kancası (DMA Hooking) veya izinsiz okuma teşebbüsü anında sistem, "Honeypot" (Tuzak) parçaları tetikler.
- **Güvenlik Mührü (Magic Marker):** Kasa, birleşim anında `HOLO_OK_` imzasını bulamazsa, saldırı altında olduğunu anlar.
- **Asitle Yıkama (Zero-Out):** Tüm bellek blokları (TLS - Thread Local Storage) mikro saniyeler içinde sıfırlanır.
- **Acil Durum İmhası:** İşletim sistemine `panic!` sinyali gönderilerek, sürecin (process) bellekteki tüm kalıntıları ile birlikte kendini sonlandırması sağlanır.

### 4. Çağrı Yığını (Call-Stack) Güvenliği
Dinamik dillerin en büyük zaafı olan `eval`, `exec` veya `os.system` gibi komutlar, kütüphane tarafından "Güvensiz Çağrı" (Unsafe Call) olarak sınıflandırılır. Kasa, kendi içine erişim talep eden çağrı yığınını (call-stack) inceleyerek, yetkisiz bir kod bloğu (RCE girişimi) tespit ettiği an kapılarını kilitler.

---

## 🧪 Red Team Denetim Raporu

Sistem, `core_engine/tests` dizininde yer alan 5 farklı "imkansız" senaryo ile test edilmiştir:

| Test Modülü | Savunma Tepkisi |
| :--- | :--- |
| **Polimorfik Kaos** | Hareketli parçalar başarıyla birleştirildi. |
| **CPU İmzalı Mühür** | RAM dökümleri okunamaz hale getirildi. |
| **Concurrency Lock** | Thread çakışmaları (Race Condition) önlendi. |
| **RCE/Eval Savunması** | Çağrı yığını denetimi ile saldırgan engellendi. |
| **DMA Bellek Kancası** | **Zehirli Hap** tetiklendi, bellek sıfırlandı. |

---

## 💻 Kullanım (Python FFI Bridge)

Kozalak, yüksek performanslı Rust çekirdeğini basit ve güvenli bir Python arayüzü ile sunar.

```python
from enclave_api import HolographicVault

# 1. Kasa Oluşturulur
vault = HolographicVault(secret=b"Yanki_BaaS_2026", num_threads=5, threshold=3)

# 2. İşlem Çalıştırma (Callback Yöntemi)
# Veri, sadece callback fonksiyonu içerisindeyken mevcuttur.
def veritabani_baglantisi(decrypted_secret):
    print(f"Bağlantı şifresi: {decrypted_secret.decode()}")

vault.execute_with(veritabani_baglantisi)
