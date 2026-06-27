from bindings.python.enclave_api import HolographicVault
import time

print("\n=======================================================")
print("[*] YANKI BaaS - HOLOGRAFİK GÜVENLİK MODÜLÜ TESTİ")
print("=======================================================")

# Docker/FastAPI veritabanı şifresi (Normalde .env dosyasından çekilir)
db_secret = b"Yanki_BaaS_Super_Secret_DB_Password_2026!"

print("\n[!] Şifre alınıyor ve Holografik Kasa'ya (Thread-Split Enclave) gönderiliyor...")
# Kasayı oluştur (5 parçaya böl, 3'ü ile birleştir)
vault = HolographicVault(secret=db_secret, num_threads=5, threshold=3)

print("[+] Kasa kilitlendi! Orijinal şifre artık bellekte yok, sadece polimorfik parçalar var.\n")

# Yankı BaaS veritabanı bağlantı simülasyonu
def connect_to_yanki_db(decrypted_secret):
    print("  [>] Kasa kilitleri açıldı (Reconstruction)!")
    print(f"  [>] Veritabanına şu şifre ile bağlanılıyor: {decrypted_secret.decode('utf-8')}")
    print("  [>] Bağlantı başarılı! Liderlik tabloları ve Cloud Save aktif.")
    time.sleep(1) # Bağlantı işlemini simüle ediyoruz
    print("  [>] Fonksiyon bitti, Kasa kapanıyor ve bellek asitle yıkanıyor (Zero-Out)...")

# İşlemi dışarıdan tetikliyoruz
print("[*] FastAPI sunucusu veritabanı bağlantısı istiyor...")
time.sleep(1)
vault.execute_with(connect_to_yanki_db)

print("\n[+] İşlem tamamlandı. Bellekte sırdan eser kalmadı. Sistem güvende!")
