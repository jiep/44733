export class Location {
    name: string;
    address: string;
    city: string;
    province: string;
    series: Array<Number>;

    constructor(name: string, address: string, city: string, province: string, series: Array<Number>) {
        this.name = name;
        this.address = address;
        this.city = city;
        this.province = province;
        this.series = series;
    }
}

