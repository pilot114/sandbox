use std::fs::File;
use ogg::{PacketReader, Packet};
use opus::{Decoder, Channels};
use std::io::BufReader;

fn dump_pck_info(p :&Packet) {
    println!(
        "Packet: serial 0x{:08x}, data {:08} large, first {: >5}, last {: >5}, absgp 0x{:016x}",
        p.stream_serial(), p.data.len(), p.first_in_page(), p.last_in_page(), p.absgp_page()
    );

    // let mut decoder = Decoder::new(48000, Channels::Mono).unwrap();
    // let frame_size = 960; // Количество сэмплов на канал, зависит от sample_rate и длительности пакета OPUS
    // let mut pcm = vec![0i16; frame_size * Channels::Mono];
    //
    // let samples = decoder.decode(&p.data, &mut pcm, false).unwrap();
    //
    // pcm.resize(samples * Channels::Mono, 0);
    //
    // println!("{:?}", pcm);
}

fn main() {
    let file = File::open("input/test_654321DmCG.ogg").unwrap();
    let reader = BufReader::new(file);

    let mut ogg_reader = PacketReader::new(reader);

    // Вектор для хранения OPUS пакетов
    let mut opus_packets = Vec::new();

    while let Ok(packet) = ogg_reader.read_packet_expected() {
        // dump_pck_info(&packet);
        opus_packets.push(packet.data);
    }
    println!("Прочитано {} OPUS пакетов.", opus_packets.len());


    let sample_rate = 48000;
    let channels = 1;

    let mut decoder = Decoder::new(sample_rate, Channels::Mono).unwrap();
    // Буфер для PCM данных
    let mut pcm_samples: Vec<i16> = Vec::new();

    for opus_packet in opus_packets {
        let mut pcm = vec![0i16; sample_rate as usize * channels];
        println!("{:?}", opus_packet.len());
        println!("{:?}", opus_packet);
        let frame_size = decoder.decode(&opus_packet, &mut pcm, false).unwrap();
        pcm_samples.extend_from_slice(&pcm[..frame_size * channels]);
        break;
    }

    // АЧХ
    // println!("{:?}", pcm_samples.len());
}