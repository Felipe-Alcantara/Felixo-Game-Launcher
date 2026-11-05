import type { Idioma } from "./i18n";

const UNIDADES = ["B", "KB", "MB", "GB", "TB"] as const;

/**
 * Tamanho legível. Usa base 1024 com rótulos "KB/MB/GB", a mesma convenção do Explorador
 * de Arquivos do Windows, para o número bater com o que o usuário vê fora do launcher.
 */
export function formatarTamanho(bytes: number | null, idioma: Idioma): string {
  if (bytes === null) return "—";
  let valor = bytes;
  let indice = 0;
  while (valor >= 1024 && indice < UNIDADES.length - 1) {
    valor /= 1024;
    indice += 1;
  }
  const casas = indice === 0 || valor >= 100 ? 0 : 1;
  const numero = new Intl.NumberFormat(idioma, {
    minimumFractionDigits: casas,
    maximumFractionDigits: casas,
  }).format(valor);
  return `${numero} ${UNIDADES[indice]}`;
}
