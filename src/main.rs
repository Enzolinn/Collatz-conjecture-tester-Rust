use std::fs::OpenOptions;
use std::io::Write;
use std::process;
use std::u128;

fn main() {
    // Número inicial vindo de argumento de linha de comando (ou 1, por padrão)
    let start: u128 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);

    // Arquivos de log
    let mut log_last = open_log("last_tested.txt");
    let mut log_fail = open_log("counterexample.txt");

    for n in start.. {
        // Sempre atualiza o último número testado
        if let Err(e) = write_log(&mut log_last, n) {
            eprintln!("Erro ao escrever last_tested.txt: {}", e);
            process::exit(1);
        }

        // Testa Collatz em n
        if let Err(err_n) = testa_collatz(n) {
            // Se der erro (overflow) durante a sequência, registra n e sai
            let _ = write_log(&mut log_fail, err_n);
            eprintln!("Erro de overflow ao testar {}. Registrado em counterexample.txt.", err_n);
            process::exit(1);
        }
    }
}

/// Abre (ou cria) um arquivo de log em modo append
fn open_log(path: &str) -> std::fs::File {
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect(&format!("Não foi possível abrir {}", path))
}

/// Escreve o número n em uma nova linha no log
fn write_log(log: &mut std::fs::File, n: u128) -> std::io::Result<()> {
    writeln!(log, "{}", n)
}

/// Testa a sequência de Collatz para `n`.
/// Em caso de overflow em qualquer passo (3*n + 1 ou n/2), retorna Err(n) — o número que causou o problema.
fn testa_collatz(mut n: u128) -> Result<(), u128> {
    // Limite de iterações só para evitar loops infinitos acidentais
    for _ in 0..10_000_000 {
        if n == 1 {
            return Ok(());
        }
        n = if n % 2 == 0 {
            n.checked_div(2).ok_or(n)?
        } else {
            // 3*n + 1, com checagem de overflow
            let tmp = n.checked_mul(3).ok_or(n)?;
            tmp.checked_add(1).ok_or(n)?
        };
    }
    // Se passou do limite de iterações, consideramos falha na conjectura
    Err(n)
}
